#![feature(test)]

//MAGIC
#[path = "../src/tea.rs"]
mod tea;

extern crate test;

use test::Bencher;
#[bench]
fn bench_tea16_encrypt(b: &mut Bencher) {
    let mut text = [0u8; 8];
    let key = [0u8; 16];
    b.iter(|| tea::tea16_encrypt(&mut text, &key));
}

#[bench]
fn bench_tea16_decrypt(b: &mut Bencher) {
    let mut text = [0u8; 8];
    let key = [0u8; 16];
    b.iter(|| tea::tea16_decrypt(&mut text, &key));
}
