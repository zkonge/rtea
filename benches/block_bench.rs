#![feature(test)]

//MAGIC
#[path = "../src/tea.rs"]
mod tea;
#[path = "../src/block.rs"]
mod block;

#[cfg(test)]
mod tests {
    extern crate test;

    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_qqtea_encrypt16(b: &mut Bencher) {
        let text = vec![0u8; 16];
        let key = vec![0u8; 16];
        b.iter(|| block::qqtea_encrypt(&text, &key));
    }
    #[bench]
    fn bench_qqtea_encrypt256(b: &mut Bencher) {
        let text = vec![0u8; 256];
        let key = vec![0u8; 16];
        b.iter(|| block::qqtea_encrypt(&text, &key));
    }
    #[bench]
    fn bench_qqtea_encrypt4096(b: &mut Bencher) {
        let text = vec![0u8; 4096];
        let key = vec![0u8; 16];
        b.iter(|| block::qqtea_encrypt(&text, &key));
    }

    #[bench]
    fn bench_qqtea_decrypt16(b: &mut Bencher) {
        let text = vec![0u8; 16];
        let key = vec![0u8; 16];
        let text = block::qqtea_encrypt(&text, &key);
        b.iter(|| block::qqtea_decrypt(&text, &key));
    }
    #[bench]
    fn bench_qqtea_decrypt256(b: &mut Bencher) {
        let text = vec![0u8; 256];
        let key = vec![0u8; 16];
        let text = block::qqtea_encrypt(&text, &key);
        b.iter(|| block::qqtea_decrypt(&text, &key));
    }
    #[bench]
    fn bench_qqtea_decrypt4096(b: &mut Bencher) {
        let text = vec![0u8; 4096];
        let key = vec![0u8; 16];
        let text = block::qqtea_encrypt(&text, &key);
        b.iter(|| block::qqtea_decrypt(&text, &key));
    }
}
