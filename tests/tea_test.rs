use std::hint::black_box;

use rtea::tea;

#[test]
fn test_tea16_encrypt() {
    let mut text = black_box([0u8; 8]);
    let key = black_box([0u8; 16]);
    text = tea::tea16_encrypt(text, key);

    assert_eq!(text, [168, 137, 247, 152, 24, 45, 128, 131]);
}

#[test]
fn test_tea16_decrypt() {
    let mut text = black_box([168, 137, 247, 152, 24, 45, 128, 131]);
    let key = black_box([0u8; 16]);
    text = tea::tea16_decrypt(text, key);

    assert_eq!(text, [0u8; 8]);
}
