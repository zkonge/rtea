//MAGIC
#[path = "../src/tea.rs"]
mod tea;

#[test]
fn test_tea16_encrypt() {
    let mut text = [0; 8];
    let key = [0; 16];
    tea::tea16_encrypt(&mut text, &key);

    assert_eq!(text, [168, 137, 247, 152, 24, 45, 128, 131]);
}

#[test]
fn test_tea16_decrypt() {
    let mut text = [168, 137, 247, 152, 24, 45, 128, 131];
    let key = [0; 16];
    tea::tea16_decrypt(&mut text, &key);

    assert_eq!(text, [0; 8]);
}
