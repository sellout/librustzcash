use alloc::vec::Vec;

pub struct TransparentOvkTestVector {
    pub c: [u8; 32],
    pub pk: [u8; 33],
    pub external_ovk: [u8; 32],
    pub internal_ovk: [u8; 32],
}

pub fn transparent_ovk() -> Vec<TransparentOvkTestVector> {
    use TransparentOvkTestVector as TestVector;

    // From https://github.com/zcash-hackworks/zcash-test-vectors/blob/master/zip_0316.py
    vec![
        TestVector {
            c: [
                0x5d, 0x7a, 0x8f, 0x73, 0x9a, 0x2d, 0x9e, 0x94, 0x5b, 0x0c, 0xe1, 0x52, 0xa8, 0x04,
                0x9e, 0x29, 0x4c, 0x4d, 0x6e, 0x66, 0xb1, 0x64, 0x93, 0x9d, 0xaf, 0xfa, 0x2e, 0xf6,
                0xee, 0x69, 0x21, 0x48,
            ],
            pk: [
                0x02, 0x16, 0x88, 0x4f, 0x1d, 0xbc, 0x92, 0x90, 0x89, 0xa4, 0x17, 0x6e, 0x84, 0x0b,
                0xb5, 0x81, 0xc8, 0x0e, 0x16, 0xe9, 0xb1, 0xab, 0xd6, 0x54, 0xe6, 0x2c, 0x8b, 0x0b,
                0x95, 0x70, 0x20, 0xb7, 0x48,
            ],
            external_ovk: [
                0xdc, 0xe7, 0xfb, 0x7f, 0x20, 0xeb, 0x77, 0x64, 0xd5, 0x12, 0x4f, 0xbd, 0x23, 0xc4,
                0xd7, 0xca, 0x8c, 0x32, 0x19, 0xec, 0x1d, 0xb3, 0xff, 0x1e, 0x08, 0x13, 0x50, 0xad,
                0x03, 0x9b, 0x40, 0x79,
            ],
            internal_ovk: [
                0x4d, 0x46, 0xc7, 0x14, 0xed, 0xda, 0xd9, 0x4a, 0x40, 0xac, 0x21, 0x28, 0x6a, 0xff,
                0x32, 0x7d, 0x7e, 0xbf, 0x11, 0x9e, 0x86, 0x85, 0x10, 0x9b, 0x44, 0xe8, 0x02, 0x83,
                0xd8, 0xc8, 0xa4, 0x00,
            ],
        },
        TestVector {
            c: [
                0xbf, 0x69, 0xb8, 0x25, 0x0c, 0x18, 0xef, 0x41, 0x29, 0x4c, 0xa9, 0x79, 0x93, 0xdb,
                0x54, 0x6c, 0x1f, 0xe0, 0x1f, 0x7e, 0x9c, 0x8e, 0x36, 0xd6, 0xa5, 0xe2, 0x9d, 0x4e,
                0x30, 0xa7, 0x35, 0x94,
            ],
            pk: [
                0x03, 0x72, 0x73, 0xb6, 0x57, 0xd9, 0x71, 0xa4, 0x5e, 0x72, 0x24, 0x0c, 0x7a, 0xaa,
                0xa7, 0xd0, 0x68, 0x5d, 0x06, 0xd7, 0x99, 0x9b, 0x0a, 0x19, 0xc4, 0xce, 0xa3, 0x27,
                0x88, 0xa6, 0xab, 0x51, 0x3d,
            ],
            external_ovk: [
                0x8d, 0x31, 0x53, 0x7b, 0x38, 0x8f, 0x40, 0x23, 0xe6, 0x48, 0x70, 0x8b, 0xfb, 0xde,
                0x2b, 0xa1, 0xff, 0x1a, 0x4e, 0xe1, 0x12, 0xea, 0x67, 0x0a, 0xd1, 0x67, 0x44, 0xf4,
                0x58, 0x3e, 0x95, 0x52,
            ],
            internal_ovk: [
                0x16, 0x77, 0x49, 0x00, 0x76, 0x9d, 0x9c, 0x03, 0xbe, 0x06, 0x32, 0x45, 0xcf, 0x1c,
                0x22, 0x44, 0xa9, 0x2e, 0x48, 0x51, 0x01, 0x54, 0x73, 0x61, 0x3f, 0xbf, 0x38, 0xd2,
                0x42, 0xd7, 0x54, 0xf6,
            ],
        },
        TestVector {
            c: [
                0x3d, 0xc1, 0x66, 0xd5, 0x6a, 0x1d, 0x62, 0xf5, 0xa8, 0xd7, 0x55, 0x1d, 0xb5, 0xfd,
                0x93, 0x13, 0xe8, 0xc7, 0x20, 0x3d, 0x99, 0x6a, 0xf7, 0xd4, 0x77, 0x08, 0x37, 0x56,
                0xd5, 0x9a, 0xf8, 0x0d,
            ],
            pk: [
                0x03, 0xec, 0x05, 0xbb, 0x7f, 0x06, 0x5e, 0x25, 0x6f, 0xf4, 0x54, 0xf8, 0xa8, 0xdf,
                0x6f, 0x2f, 0x9b, 0x8a, 0x8c, 0x95, 0x08, 0xca, 0xac, 0xfe, 0xe9, 0x52, 0x1c, 0xbe,
                0x68, 0x9d, 0xd1, 0x12, 0x0f,
            ],
            external_ovk: [
                0xdb, 0x97, 0x52, 0x0e, 0x2f, 0xe3, 0x68, 0xad, 0x50, 0x2d, 0xef, 0xf8, 0x42, 0xf0,
                0xc0, 0xee, 0x5d, 0x20, 0x3b, 0x48, 0x33, 0x7a, 0x0f, 0xff, 0x75, 0xbe, 0x24, 0x52,
                0x59, 0x77, 0xf3, 0x7e,
            ],
            internal_ovk: [
                0xbc, 0x4a, 0xcb, 0x5f, 0x52, 0xb8, 0xae, 0x21, 0xe3, 0x32, 0xb1, 0x7c, 0x29, 0x63,
                0x1f, 0x68, 0xe9, 0x68, 0x2a, 0x46, 0xc4, 0xa7, 0xab, 0xc8, 0xed, 0xf9, 0x0d, 0x37,
                0xae, 0xea, 0xd3, 0x6c,
            ],
        },
        TestVector {
            c: [
                0x49, 0x5c, 0x22, 0x2f, 0x7f, 0xba, 0x1e, 0x31, 0xde, 0xfa, 0x3d, 0x5a, 0x57, 0xef,
                0xc2, 0xe1, 0xe9, 0xb0, 0x1a, 0x03, 0x55, 0x87, 0xd5, 0xfb, 0x1a, 0x38, 0xe0, 0x1d,
                0x94, 0x90, 0x3d, 0x3c,
            ],
            pk: [
                0x02, 0x81, 0x8f, 0x50, 0xce, 0x47, 0x10, 0xf4, 0xeb, 0x11, 0xe7, 0x43, 0xe6, 0x40,
                0x85, 0x44, 0xaa, 0x3c, 0x12, 0x3c, 0x7f, 0x07, 0xe2, 0xaa, 0xbb, 0x91, 0xaf, 0xc4,
                0xec, 0x48, 0x78, 0x8d, 0xe9,
            ],
            external_ovk: [
                0xb8, 0xa3, 0x6d, 0x62, 0xa6, 0x3f, 0x69, 0x36, 0x7b, 0xe3, 0xf4, 0xbe, 0xd4, 0x20,
                0x26, 0x4a, 0xdb, 0x63, 0x7b, 0xbb, 0x47, 0x0e, 0x1f, 0x56, 0xe0, 0x33, 0x8b, 0x38,
                0xe2, 0xa6, 0x90, 0x97,
            ],
            internal_ovk: [
                0x4f, 0xf6, 0xfa, 0xf2, 0x06, 0x63, 0x1e, 0xcb, 0x01, 0xf9, 0x57, 0x30, 0xf7, 0xe5,
                0x5b, 0xfc, 0xff, 0x8b, 0x02, 0xa3, 0x14, 0x88, 0x5a, 0x6d, 0x24, 0x8e, 0x6e, 0xbe,
                0xb7, 0x4d, 0x3e, 0x50,
            ],
        },
        TestVector {
            c: [
                0xa7, 0xaf, 0x9d, 0xb6, 0x99, 0x0e, 0xd8, 0x3d, 0xd6, 0x4a, 0xf3, 0x59, 0x7c, 0x04,
                0x32, 0x3e, 0xa5, 0x1b, 0x00, 0x52, 0xad, 0x80, 0x84, 0xa8, 0xb9, 0xda, 0x94, 0x8d,
                0x32, 0x0d, 0xad, 0xd6,
            ],
            pk: [
                0x02, 0xae, 0x36, 0xb6, 0x1a, 0x3d, 0x10, 0xf1, 0xaa, 0x75, 0x2a, 0xb1, 0xdc, 0x16,
                0xe3, 0xe4, 0x9b, 0x6a, 0xc0, 0xd2, 0xae, 0x19, 0x07, 0xd2, 0xe6, 0x94, 0x25, 0xec,
                0x12, 0xc9, 0x3a, 0xae, 0xbc,
            ],
            external_ovk: [
                0xda, 0x6f, 0x47, 0x0f, 0x42, 0x5b, 0x3d, 0x27, 0xf4, 0x28, 0x6e, 0xf0, 0x3b, 0x7e,
                0x87, 0x01, 0x7c, 0x20, 0xa7, 0x10, 0xb3, 0xff, 0xb9, 0xc1, 0xb6, 0x6c, 0x71, 0x60,
                0x92, 0xe3, 0xd9, 0xbc,
            ],
            internal_ovk: [
                0x09, 0xb5, 0x4f, 0x75, 0xcb, 0x70, 0x32, 0x67, 0x1d, 0xc6, 0x8a, 0xaa, 0x07, 0x30,
                0x5f, 0x38, 0xcd, 0xbc, 0x87, 0x9e, 0xe1, 0x5b, 0xec, 0x04, 0x71, 0x3c, 0x24, 0xdc,
                0xe3, 0xca, 0x70, 0x26,
            ],
        },
        TestVector {
            c: [
                0xe0, 0x0c, 0x7a, 0x1d, 0x48, 0xaf, 0x04, 0x68, 0x27, 0x59, 0x1e, 0x97, 0x33, 0xa9,
                0x7f, 0xa6, 0xb6, 0x79, 0xf3, 0xdc, 0x60, 0x1d, 0x00, 0x82, 0x85, 0xed, 0xcb, 0xda,
                0xe6, 0x9c, 0xe8, 0xfc,
            ],
            pk: [
                0x02, 0x49, 0x26, 0x53, 0x80, 0xd2, 0xb0, 0x2e, 0x0a, 0x1d, 0x98, 0x8f, 0x3d, 0xe3,
                0x45, 0x8b, 0x6e, 0x00, 0x29, 0x1d, 0xb0, 0xe6, 0x2e, 0x17, 0x47, 0x91, 0xd0, 0x09,
                0x29, 0x9f, 0x61, 0xfe, 0xc4,
            ],
            external_ovk: [
                0x60, 0xa7, 0xa0, 0x8e, 0xef, 0xa2, 0x4e, 0x75, 0xcc, 0xbb, 0x29, 0xdc, 0x84, 0x94,
                0x67, 0x2d, 0x73, 0x0f, 0xb3, 0x88, 0x7c, 0xb2, 0x6e, 0xf5, 0x1c, 0x6a, 0x1a, 0x78,
                0xe8, 0x8a, 0x78, 0x39,
            ],
            internal_ovk: [
                0x3b, 0xab, 0x40, 0x98, 0x08, 0x10, 0x8b, 0xa9, 0xe5, 0xa1, 0xbb, 0x6a, 0x42, 0x24,
                0x59, 0x9d, 0x62, 0xcc, 0xee, 0x63, 0xff, 0x2f, 0x38, 0x15, 0x4c, 0x7f, 0xb0, 0xc9,
                0xa9, 0xa5, 0x79, 0x0f,
            ],
        },
        TestVector {
            c: [
                0xe2, 0x88, 0x53, 0x15, 0xeb, 0x46, 0x71, 0x09, 0x8b, 0x79, 0x53, 0x5e, 0x79, 0x0f,
                0xe5, 0x3e, 0x29, 0xfe, 0xf2, 0xb3, 0x76, 0x66, 0x97, 0xac, 0x32, 0xb4, 0xf4, 0x73,
                0xf4, 0x68, 0xa0, 0x08,
            ],
            pk: [
                0x03, 0x9a, 0x0e, 0x46, 0x39, 0xb4, 0x69, 0x1f, 0x02, 0x7c, 0x0d, 0xb7, 0xfe, 0xf1,
                0xbb, 0x5e, 0xf9, 0x0a, 0xcd, 0xb7, 0x08, 0x62, 0x6d, 0x2e, 0x1f, 0x3e, 0x38, 0x3e,
                0xe7, 0x5b, 0x31, 0xcf, 0x57,
            ],
            external_ovk: [
                0xbb, 0x47, 0x87, 0x2c, 0x25, 0x09, 0xbf, 0x3c, 0x72, 0xde, 0xdf, 0x4f, 0xc1, 0x77,
                0x0f, 0x91, 0x93, 0xe2, 0xc1, 0x90, 0xd7, 0xaa, 0x8e, 0x9e, 0x88, 0x1a, 0xd2, 0xf1,
                0x73, 0x48, 0x4e, 0xf2,
            ],
            internal_ovk: [
                0x5f, 0x36, 0xdf, 0xa3, 0x6c, 0xa7, 0x65, 0x74, 0x50, 0x29, 0x4e, 0xaa, 0xdd, 0xad,
                0x78, 0xaf, 0xf2, 0xb3, 0xdc, 0x38, 0x5a, 0x57, 0x73, 0x5a, 0xc0, 0x0d, 0x3d, 0x9a,
                0x29, 0x2b, 0x8c, 0x77,
            ],
        },
        TestVector {
            c: [
                0xed, 0x94, 0x94, 0xc6, 0xac, 0x89, 0x3c, 0x49, 0x72, 0x38, 0x33, 0xec, 0x89, 0x26,
                0xc1, 0x03, 0x95, 0x86, 0xa7, 0xaf, 0xcf, 0x4a, 0x0d, 0x9c, 0x73, 0x1e, 0x98, 0x5d,
                0x99, 0x58, 0x9c, 0x8b,
            ],
            pk: [
                0x03, 0xbb, 0xf4, 0x49, 0x82, 0xf1, 0xba, 0x3a, 0x2b, 0x9d, 0xd3, 0xc1, 0x77, 0x4d,
                0x71, 0xce, 0x33, 0x60, 0x59, 0x9b, 0x07, 0xf2, 0x11, 0xc8, 0x16, 0xb8, 0xc4, 0x3b,
                0x98, 0x42, 0x23, 0x09, 0x24,
            ],
            external_ovk: [
                0xed, 0xe8, 0xfb, 0x11, 0x37, 0x9b, 0x15, 0xae, 0xc4, 0xfa, 0x4e, 0xc5, 0x12, 0x4c,
                0x95, 0x00, 0xad, 0xf4, 0x0e, 0xb6, 0xf7, 0xca, 0xa5, 0xe9, 0xce, 0x80, 0xf6, 0xbd,
                0x9e, 0x73, 0xd0, 0xe7,
            ],
            internal_ovk: [
                0x25, 0x0b, 0x4d, 0xfc, 0x34, 0xdd, 0x57, 0x76, 0x74, 0x51, 0x57, 0xf3, 0x82, 0xce,
                0x6d, 0xe4, 0xf6, 0xfe, 0x22, 0xd7, 0x98, 0x02, 0xf3, 0x9f, 0xe1, 0x34, 0x77, 0x8b,
                0x79, 0x40, 0x42, 0xd3,
            ],
        },
        TestVector {
            c: [
                0x92, 0x47, 0x69, 0x30, 0xd0, 0x69, 0x89, 0x6c, 0xff, 0x30, 0xeb, 0x41, 0x4f, 0x72,
                0x7b, 0x89, 0xe0, 0x01, 0xaf, 0xa2, 0xfb, 0x8d, 0xc3, 0x43, 0x6d, 0x75, 0xa4, 0xa6,
                0xf2, 0x65, 0x72, 0x50,
            ],
            pk: [
                0x03, 0xff, 0x63, 0xc7, 0x89, 0x25, 0x1c, 0x10, 0x43, 0xc6, 0xf9, 0x6c, 0x66, 0xbf,
                0x5b, 0x0f, 0x61, 0xc9, 0xd6, 0x5f, 0xef, 0x5a, 0xaf, 0x42, 0x84, 0xa6, 0xa5, 0x69,
                0x94, 0x94, 0x1c, 0x05, 0xfa,
            ],
            external_ovk: [
                0xb3, 0x11, 0x52, 0x06, 0x42, 0x71, 0x01, 0x01, 0xbb, 0xc8, 0x1b, 0xbe, 0x92, 0x85,
                0x1f, 0x9e, 0x65, 0x36, 0x22, 0x3e, 0xd6, 0xe6, 0xa1, 0x28, 0x59, 0x06, 0x62, 0x1e,
                0xfa, 0xe6, 0x41, 0x10,
            ],
            internal_ovk: [
                0xf4, 0x46, 0xc0, 0xc1, 0x74, 0x1c, 0x94, 0x42, 0x56, 0x8e, 0x12, 0xf0, 0x55, 0xef,
                0xd5, 0x0c, 0x1e, 0xfe, 0x4d, 0x71, 0x53, 0x3d, 0x97, 0x6b, 0x08, 0xe9, 0x94, 0x41,
                0x44, 0x49, 0xc4, 0xac,
            ],
        },
        TestVector {
            c: [
                0x7d, 0x41, 0x7a, 0xdb, 0x3d, 0x15, 0xcc, 0x54, 0xdc, 0xb1, 0xfc, 0xe4, 0x67, 0x50,
                0x0c, 0x6b, 0x8f, 0xb8, 0x6b, 0x12, 0xb5, 0x6d, 0xa9, 0xc3, 0x82, 0x85, 0x7d, 0xee,
                0xcc, 0x40, 0xa9, 0x8d,
            ],
            pk: [
                0x02, 0xbf, 0x39, 0x20, 0xce, 0x2e, 0x9e, 0x95, 0xb0, 0xee, 0xce, 0x13, 0x0a, 0x50,
                0xba, 0x7d, 0xcc, 0x6f, 0x26, 0x51, 0x2a, 0x9f, 0xc7, 0xb8, 0x04, 0xaf, 0xf0, 0x89,
                0xf5, 0x0c, 0xbc, 0xff, 0xf7,
            ],
            external_ovk: [
                0xae, 0x63, 0x84, 0xf8, 0x07, 0x72, 0x1c, 0x5f, 0x46, 0xc8, 0xaa, 0x83, 0x3b, 0x66,
                0x9b, 0x01, 0xc4, 0x22, 0x7c, 0x00, 0x18, 0xcb, 0x27, 0x29, 0xa9, 0x79, 0x91, 0x01,
                0xea, 0xb8, 0x5a, 0xb9,
            ],
            internal_ovk: [
                0xef, 0x70, 0x8e, 0xb8, 0x26, 0xd8, 0xbf, 0xcd, 0x7f, 0xaa, 0x4f, 0x90, 0xdf, 0x46,
                0x1d, 0xed, 0x08, 0xd1, 0x6e, 0x19, 0x1b, 0x4e, 0x51, 0xb8, 0xa3, 0xa9, 0x1c, 0x02,
                0x0b, 0x32, 0xcc, 0x07,
            ],
        },
    ]
}
