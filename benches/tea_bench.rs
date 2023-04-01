#![feature(test)]

extern crate test;

use std::hint::black_box;

use rtea::tea;
use test::Bencher;

#[bench]
fn bench_tea16_encrypt(b: &mut Bencher) {
    b.bytes = 8;
    let text = black_box([0u8; 8]);
    let key = black_box([0u8; 16]);
    b.iter(|| tea::tea16_encrypt(text, key));
}

#[bench]
fn bench_tea16_decrypt(b: &mut Bencher) {
    b.bytes = 8;
    let text = black_box([0u8; 8]);
    let key = black_box([0u8; 16]);
    b.iter(|| tea::tea16_decrypt(text, key));
}
