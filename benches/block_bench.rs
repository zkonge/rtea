#![feature(test)]

extern crate test;

use rtea::block;
use test::Bencher;

#[bench]
fn bench_qqtea_encrypt16(b: &mut Bencher) {
    b.bytes = 16;
    let text = [0u8; 16];
    let key = [0u8; 16];
    b.iter(|| block::qqtea_encrypt(&text, key));
}

#[bench]
fn bench_qqtea_encrypt256(b: &mut Bencher) {
    b.bytes = 256;
    let text = [0u8; 256];
    let key = [0u8; 16];
    b.iter(|| block::qqtea_encrypt(&text, key));
}

#[bench]
fn bench_qqtea_encrypt4096(b: &mut Bencher) {
    b.bytes = 4096;
    let text = [0u8; 4096];
    let key = [0u8; 16];
    b.iter(|| block::qqtea_encrypt(&text, key));
}

#[bench]
fn bench_qqtea_decrypt16(b: &mut Bencher) {
    b.bytes = 16;
    let text = [0u8; 16];
    let key = [0u8; 16];
    let mut text = block::qqtea_encrypt(&text, key);
    b.iter(|| block::qqtea_decrypt(&mut text, key));
}

#[bench]
fn bench_qqtea_decrypt256(b: &mut Bencher) {
    b.bytes = 256;
    let text = [0u8; 256];
    let key = [0u8; 16];
    let mut text = block::qqtea_encrypt(&text, key);
    b.iter(|| block::qqtea_decrypt(&mut text, key));
}

#[bench]
fn bench_qqtea_decrypt4096(b: &mut Bencher) {
    b.bytes = 4096;
    let text = [0u8; 4096];
    let key = [0u8; 16];
    let mut text = block::qqtea_encrypt(&text, key);
    b.iter(|| block::qqtea_decrypt(&mut text, key));
}

#[bench]
fn bench_qqtea_encrypt16_noalloc(b: &mut Bencher) {
    b.bytes = 16;
    let text = [0u8; 16];
    let mut out = [0u8; 32];
    let key = [0u8; 16];
    let cipher = block::QQTea::new(key);
    b.iter(|| cipher.encrypt_inout(&text, &mut out));
}

#[bench]
fn bench_qqtea_encrypt256_noalloc(b: &mut Bencher) {
    b.bytes = 256;
    let text = [0u8; 256];
    let mut out = [0u8; 256 + 16];
    let key = [0u8; 16];
    let cipher = block::QQTea::new(key);
    b.iter(|| cipher.encrypt_inout(&text, &mut out));
}

#[bench]
fn bench_qqtea_encrypt4096_noalloc(b: &mut Bencher) {
    b.bytes = 4096;
    let text = [0u8; 4096];
    let mut out = [0u8; 4096 + 16];
    let key = [0u8; 16];
    let cipher = block::QQTea::new(key);
    b.iter(|| cipher.encrypt_inout(&text, &mut out));
}
