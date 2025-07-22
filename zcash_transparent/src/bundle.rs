//! Structs representing the components within Zcash transactions.

use alloc::vec::Vec;
use core::fmt::Debug;
use core2::io::{self, Read, Write};

use zcash_protocol::{
    value::{BalanceError, ZatBalance, Zatoshis},
    TxId,
};
use zcash_script::{opcode::Opcode, script};

use crate::{
    address::{
        read_script_pubkey, read_script_sig, write_script_pubkey, write_script_sig,
        TransparentAddress,
    },
    sighash::TransparentAuthorizingContext,
};

pub trait Authorization: Debug {
    type ScriptSig: Debug + Clone + PartialEq;
}

/// Marker type for a bundle that contains no authorizing data, and the necessary input
/// information for creating sighashes.
#[derive(Debug)]
pub struct EffectsOnly {
    pub(crate) inputs: Vec<TxOut>,
}

impl Authorization for EffectsOnly {
    type ScriptSig = ();
}

impl TransparentAuthorizingContext for EffectsOnly {
    fn input_amounts(&self) -> Vec<Zatoshis> {
        self.inputs.iter().map(|input| input.value).collect()
    }

    fn input_scriptpubkeys(&self) -> Vec<script::PubKey> {
        self.inputs
            .iter()
            .map(|input| input.script_pubkey.clone())
            .collect()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Authorized;

impl Authorization for Authorized {
    type ScriptSig = script::Sig<Opcode>;
}

pub trait MapAuth<A: Authorization, B: Authorization> {
    fn map_script_sig(&self, s: A::ScriptSig) -> B::ScriptSig;
    fn map_authorization(&self, s: A) -> B;
}

/// The identity map.
impl MapAuth<Authorized, Authorized> for () {
    fn map_script_sig(
        &self,
        s: <Authorized as Authorization>::ScriptSig,
    ) -> <Authorized as Authorization>::ScriptSig {
        s
    }

    fn map_authorization(&self, s: Authorized) -> Authorized {
        s
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Bundle<A: Authorization> {
    pub vin: Vec<TxIn<A>>,
    pub vout: Vec<TxOut>,
    pub authorization: A,
}

impl<A: Authorization> Bundle<A> {
    /// Returns `true` if this bundle matches the definition of a coinbase transaction.
    ///
    /// Note that this is defined purely in terms of the transparent transaction part. The
    /// consensus rules enforce additional rules on the shielded parts (namely, that they
    /// don't have any inputs) of transactions with a transparent part that matches this
    /// definition.
    pub fn is_coinbase(&self) -> bool {
        // From `CTransaction::IsCoinBase()` in zcashd:
        //   return (vin.size() == 1 && vin[0].prevout.IsNull());
        matches!(&self.vin[..], [input] if input.prevout.is_null())
    }

    pub fn map_authorization<B: Authorization, F: MapAuth<A, B>>(self, f: F) -> Bundle<B> {
        Bundle {
            vin: self
                .vin
                .into_iter()
                .map(|txin| TxIn {
                    prevout: txin.prevout,
                    script_sig: f.map_script_sig(txin.script_sig),
                    sequence: txin.sequence,
                })
                .collect(),
            vout: self.vout,
            authorization: f.map_authorization(self.authorization),
        }
    }

    /// The amount of value added to or removed from the transparent pool by the action of this
    /// bundle. A positive value represents that the containing transaction has funds being
    /// transferred out of the transparent pool into shielded pools or to fees; a negative value
    /// means that the containing transaction has funds being transferred into the transparent pool
    /// from the shielded pools.
    pub fn value_balance<E, F>(&self, mut get_prevout_value: F) -> Result<Option<ZatBalance>, E>
    where
        E: From<BalanceError>,
        F: FnMut(&OutPoint) -> Result<Option<Zatoshis>, E>,
    {
        let mut input_sum = Zatoshis::ZERO;
        for txin in &self.vin {
            if let Some(v) = get_prevout_value(&txin.prevout)? {
                input_sum = (input_sum + v).ok_or(BalanceError::Overflow)?;
            } else {
                return Ok(None);
            }
        }

        let output_sum = self
            .vout
            .iter()
            .map(|p| ZatBalance::from(p.value))
            .sum::<Option<ZatBalance>>()
            .ok_or(BalanceError::Overflow)?;

        let balance = (ZatBalance::from(input_sum) - output_sum).ok_or(BalanceError::Underflow)?;
        Ok(Some(balance))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct OutPoint {
    pub(crate) hash: TxId,
    pub(crate) n: u32,
}

impl OutPoint {
    /// Constructs an `OutPoint` for the output at index `n` in the transaction
    /// with txid `hash`.
    pub fn new(hash: [u8; 32], n: u32) -> Self {
        OutPoint {
            hash: TxId::from_bytes(hash),
            n,
        }
    }

    /// Constructs a fake `OutPoint` for use in tests.
    #[cfg(any(test, feature = "test-dependencies"))]
    pub const fn fake() -> Self {
        OutPoint {
            hash: TxId::from_bytes([1u8; 32]),
            n: 1,
        }
    }

    pub fn read<R: Read>(mut reader: R) -> io::Result<Self> {
        let mut hash = [0u8; 32];
        reader.read_exact(&mut hash)?;
        let mut n = [0; 4];
        reader.read_exact(&mut n)?;
        Ok(OutPoint::new(hash, u32::from_le_bytes(n)))
    }

    pub fn write<W: Write>(&self, mut writer: W) -> io::Result<()> {
        writer.write_all(self.hash.as_ref())?;
        writer.write_all(&self.n.to_le_bytes())
    }

    /// Returns `true` if this `OutPoint` is "null" in the Bitcoin sense: it has txid set to
    /// all-zeroes and output index set to `u32::MAX`.
    fn is_null(&self) -> bool {
        // From `BaseOutPoint::IsNull()` in zcashd:
        //   return (hash.IsNull() && n == (uint32_t) -1);
        self.hash.is_null() && self.n == u32::MAX
    }

    /// Returns the output index of this `OutPoint`.
    pub fn n(&self) -> u32 {
        self.n
    }

    /// Returns the byte representation of the txid of the transaction containing this `OutPoint`.
    pub fn hash(&self) -> &[u8; 32] {
        self.hash.as_ref()
    }

    /// Returns the txid of the transaction containing this `OutPoint`.
    pub fn txid(&self) -> &TxId {
        &self.hash
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TxIn<A: Authorization> {
    pub prevout: OutPoint,
    pub script_sig: A::ScriptSig,
    pub sequence: u32,
}

impl TxIn<Authorized> {
    pub fn read<R: Read>(mut reader: &mut R) -> io::Result<Self> {
        let prevout = OutPoint::read(&mut reader)?;
        let script_sig = read_script_sig(&mut reader)?;
        let sequence = {
            let mut sequence = [0; 4];
            reader.read_exact(&mut sequence)?;
            u32::from_le_bytes(sequence)
        };

        Ok(TxIn {
            prevout,
            script_sig,
            sequence,
        })
    }

    pub fn write<W: Write>(&self, mut writer: W) -> io::Result<()> {
        self.prevout.write(&mut writer)?;
        write_script_sig(&self.script_sig, &mut writer)?;
        writer.write_all(&self.sequence.to_le_bytes())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TxOut {
    pub value: Zatoshis,
    pub script_pubkey: script::PubKey,
}

impl TxOut {
    // Constructs a new `TxOut` from its constituent parts.
    pub fn new(value: Zatoshis, script_pubkey: script::PubKey) -> Self {
        Self {
            value,
            script_pubkey,
        }
    }

    pub fn read<R: Read>(mut reader: &mut R) -> io::Result<Self> {
        let value = {
            let mut tmp = [0u8; 8];
            reader.read_exact(&mut tmp)?;
            Zatoshis::from_nonnegative_i64_le_bytes(tmp)
        }
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "value out of range"))?;
        let script_pubkey = read_script_pubkey(&mut reader)?;

        Ok(TxOut {
            value,
            script_pubkey,
        })
    }

    pub fn write<W: Write>(&self, mut writer: W) -> io::Result<()> {
        writer.write_all(&self.value.to_i64_le_bytes())?;
        write_script_pubkey(&self.script_pubkey, &mut writer)
    }

    /// Returns the address to which the TxOut was sent, if this is a valid P2SH or P2PKH output.
    pub fn recipient_address(&self) -> Option<TransparentAddress> {
        TransparentAddress::from_script_pubkey(&self.script_pubkey)
    }
}

#[cfg(any(test, feature = "test-dependencies"))]
pub mod testing {
    use proptest::collection::vec;
    use proptest::prelude::*;
    use proptest::sample::select;
    use zcash_protocol::value::testing::arb_zatoshis;
    use zcash_script::{
        opcode::Opcode,
        script::{self, Parsable},
    };

    use super::{Authorized, Bundle, OutPoint, TxIn, TxOut};

    pub const VALID_OPCODES: [u8; 8] = [
        0x00, // OP_0,
        0x51, // OP_1,
        0x52, // OP_2,
        0x53, // OP_3,
        0xac, // OP_CHECKSIG,
        0x63, // OP_IF,
        0x65, // OP_VERIF,
        0x6a, // OP_RETURN,
    ];

    prop_compose! {
        pub fn arb_outpoint()(hash in prop::array::uniform32(0u8..), n in 0..100u32) -> OutPoint {
            OutPoint::new(hash, n)
        }
    }

    prop_compose! {
        pub fn arb_script_sig()(v in vec(select(&VALID_OPCODES[..]), 1..256)) -> script::Sig<Opcode> {
            script::Sig::from_bytes(&v).expect("valid script_sig").0
        }
    }

    prop_compose! {
        pub fn arb_script_pubkey()(v in vec(select(&VALID_OPCODES[..]), 1..256)) -> script::PubKey {
            script::PubKey::from_bytes(&v).expect("valid script_pubkey").0
        }
    }

    prop_compose! {
        pub fn arb_txin()(
            prevout in arb_outpoint(),
            script_sig in arb_script_sig(),
            sequence in any::<u32>()
        ) -> TxIn<Authorized> {
            TxIn { prevout, script_sig, sequence }
        }
    }

    prop_compose! {
        pub fn arb_txout()(value in arb_zatoshis(), script_pubkey in arb_script_pubkey()) -> TxOut {
            TxOut { value, script_pubkey }
        }
    }

    prop_compose! {
        pub fn arb_bundle()(
            vin in vec(arb_txin(), 0..10),
            vout in vec(arb_txout(), 0..10),
        ) -> Option<Bundle<Authorized>> {
            if vin.is_empty() && vout.is_empty() {
                None
            } else {
                Some(Bundle { vin, vout, authorization: Authorized })
            }
        }
    }
}
