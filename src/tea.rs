const TEA_DELTA: u32 = 0x9E3779B9;

#[derive(Clone, Copy)]
pub struct Tea16 {
    key: [u32; 4],
}

impl Tea16 {
    #[inline]
    pub fn encrypt(&self, n: u64) -> u64 {
        let mut sum: u32 = 0;
        let (mut x, mut y) = ((n >> 32) as u32, n as u32);
        let [k0, k1, k2, k3] = self.key;

        for _ in 0..16 {
            sum = sum.wrapping_add(TEA_DELTA);
            x = x.wrapping_add(
                k0.wrapping_add(y << 4) ^ y.wrapping_add(sum) ^ k1.wrapping_add(y >> 5),
            );
            y = y.wrapping_add(
                k2.wrapping_add(x << 4) ^ x.wrapping_add(sum) ^ k3.wrapping_add(x >> 5),
            );
        }

        ((x as u64) << 32) | y as u64
    }

    #[inline]
    pub fn decrypt(&self, n: u64) -> u64 {
        let mut sum: u32 = TEA_DELTA << 4;
        let (mut x, mut y) = ((n >> 32) as u32, n as u32);
        let [k0, k1, k2, k3] = self.key;

        for _ in 0..16 {
            y = y.wrapping_sub(
                k2.wrapping_add(x << 4) ^ x.wrapping_add(sum) ^ k3.wrapping_add(x >> 5),
            );
            x = x.wrapping_sub(
                k0.wrapping_add(y << 4) ^ y.wrapping_add(sum) ^ k1.wrapping_add(y >> 5),
            );
            sum = sum.wrapping_sub(TEA_DELTA);
        }

        ((x as u64) << 32) | y as u64
    }

    #[inline]
    pub fn new(key: [u8; 16]) -> Self {
        Self {
            key: [
                u32::from_be_bytes(key[0..4].try_into().unwrap()),
                u32::from_be_bytes(key[4..8].try_into().unwrap()),
                u32::from_be_bytes(key[8..12].try_into().unwrap()),
                u32::from_be_bytes(key[12..16].try_into().unwrap()),
            ],
        }
    }
}

pub fn tea16_encrypt(data: [u8; 8], key: [u8; 16]) -> [u8; 8] {
    let n = u64::from_be_bytes(data);
    Tea16::new(key).encrypt(n).to_be_bytes()
}

pub fn tea16_decrypt(data: [u8; 8], key: [u8; 16]) -> [u8; 8] {
    let n = u64::from_be_bytes(data);
    Tea16::new(key).decrypt(n).to_be_bytes()
}
