use crate::KupynaH;
use crate::sub_units::t_xor_plus::{t_plus_l, t_xor_l};

fn setup_hash_params() -> KupynaH {
    KupynaH::default()
}

#[test]
fn test_t_xor_l() {
    let input = [
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
        0x0F, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D,
        0x1E, 0x1F, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2A, 0x2B, 0x2C,
        0x2D, 0x2E, 0x2F, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3A, 0x3B,
        0x3C, 0x3D, 0x3E, 0x3F, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4A,
        0x4B, 0x4C, 0x4D, 0x4E, 0x4F, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59,
        0x5A, 0x5B, 0x5C, 0x5D, 0x5E, 0x5F, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68,
        0x69, 0x6A, 0x6B, 0x6C, 0x6D, 0x6E, 0x6F, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77,
        0x78, 0x79, 0x7A, 0x7B, 0x7C, 0x7D, 0x7E, 0x7F,
    ];

    let expected_output = [
        0x60, 0x4B, 0x9D, 0xCF, 0x7E, 0xAA, 0x57, 0x85, 0x94, 0xD1, 0x83, 0xEE, 0xF2, 0xDD, 0x97,
        0xA3, 0x2C, 0x11, 0x1C, 0x81, 0x70, 0xC0, 0xA5, 0x08, 0x6A, 0x08, 0xC9, 0xE4, 0x28, 0x81,
        0x11, 0x32, 0x31, 0xBE, 0xC7, 0xB7, 0x1D, 0x0E, 0xE3, 0x1D, 0xE8, 0x36, 0x3B, 0x4A, 0xA6,
        0xAF, 0x89, 0x0B, 0xDE, 0xEE, 0x5C, 0x96, 0x66, 0x3A, 0x44, 0x38, 0x3A, 0x40, 0x09, 0x30,
        0x60, 0xE7, 0x65, 0x15, 0x2D, 0xEB, 0xEC, 0xD2, 0x5B, 0x83, 0x42, 0xC4, 0xEF, 0x4E, 0x75,
        0x0F, 0xC3, 0xF4, 0x81, 0x4F, 0xA9, 0xE1, 0xD1, 0x1F, 0xE7, 0xF6, 0xF8, 0xCF, 0x32, 0x72,
        0xE7, 0xE1, 0x61, 0x4F, 0x91, 0xAD, 0x6F, 0x01, 0xF7, 0x28, 0xD8, 0xDB, 0xBE, 0x1F, 0x2A,
        0xC1, 0x97, 0x77, 0x1E, 0x37, 0x8F, 0x8D, 0xD7, 0xD1, 0x31, 0x32, 0x7B, 0xF1, 0xA9, 0x43,
        0xA9, 0x55, 0xF1, 0xF7, 0xC8, 0x32, 0xAD, 0xF3,
    ];

    let hash_params = setup_hash_params();
    let result = t_xor_l(&input, &hash_params);
    assert_eq!(result, expected_output);
}

#[test]
fn test_t_plus_l() {
    let input = [
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
        0x0F, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D,
        0x1E, 0x1F, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2A, 0x2B, 0x2C,
        0x2D, 0x2E, 0x2F, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3A, 0x3B,
        0x3C, 0x3D, 0x3E, 0x3F, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4A,
        0x4B, 0x4C, 0x4D, 0x4E, 0x4F, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59,
        0x5A, 0x5B, 0x5C, 0x5D, 0x5E, 0x5F, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68,
        0x69, 0x6A, 0x6B, 0x6C, 0x6D, 0x6E, 0x6F, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77,
        0x78, 0x79, 0x7A, 0x7B, 0x7C, 0x7D, 0x7E, 0x7F,
    ];

    let expected_output = [
        0x36, 0x57, 0x5D, 0x99, 0x30, 0x36, 0xAF, 0xDE, 0xB2, 0x65, 0x4C, 0x1E, 0x13, 0x66, 0x0A,
        0x9D, 0x4F, 0x0E, 0x10, 0x5C, 0xA2, 0x33, 0x6F, 0x2B, 0xB3, 0x69, 0x00, 0x45, 0x25, 0x9A,
        0x1A, 0x9D, 0x3F, 0x24, 0x85, 0x07, 0xC3, 0x42, 0xA7, 0x0B, 0x42, 0xF7, 0x49, 0x81, 0xEC,
        0xE4, 0x6D, 0xD0, 0x5E, 0x1D, 0x30, 0x9F, 0x77, 0x4E, 0x1E, 0xD2, 0x13, 0x24, 0x7C, 0xC8,
        0x21, 0x46, 0x16, 0x73, 0xC7, 0x41, 0x9A, 0xE1, 0x2B, 0x93, 0x61, 0xF3, 0x2C, 0x75, 0x38,
        0xC1, 0x59, 0x09, 0xB1, 0x97, 0xE2, 0x0F, 0x9E, 0x09, 0xDD, 0x28, 0xCD, 0xD4, 0xD7, 0xC2,
        0x34, 0xDB, 0xDB, 0x47, 0x93, 0x18, 0xA2, 0x58, 0xA7, 0x18, 0x0B, 0x18, 0x33, 0x17, 0x8A,
        0x20, 0xFC, 0xFE, 0x05, 0xA6, 0x06, 0x4F, 0xD7, 0xB1, 0xEA, 0x96, 0x07, 0x99, 0x5E, 0x98,
        0xD9, 0x0D, 0x2D, 0x55, 0xDC, 0xF7, 0x2F, 0x5F,
    ];

    let hash_params = setup_hash_params();
    let result = t_plus_l(&input, &hash_params);
    assert_eq!(result, expected_output);
}
