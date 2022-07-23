const TEA_DELTA: u32 = 0x9E3779B9;

pub struct Tea16 {
    key: [u32; 4],
}

impl Tea16 {
    #[inline]
    pub fn encrypt(&self, n: u64) -> u64 {
        let mut sum: u32 = 0;
        let (mut x, mut y) = ((n >> 32) as u32, n as u32);
        let k0 = self.key[0];
        let k1 = self.key[1];
        let k2 = self.key[2];
        let k3 = self.key[3];

        for _ in 0..16 {
            sum = sum.wrapping_add(TEA_DELTA);
            x = x.wrapping_add(k0.wrapping_add(y << 4) ^ y.wrapping_add(sum) ^ k1.wrapping_add(y >> 5));
            y = y.wrapping_add(k2.wrapping_add(x << 4) ^ x.wrapping_add(sum) ^ k3.wrapping_add(x >> 5));
        }

        ((x as u64) << 32) | y as u64
    }

    #[inline]
    pub fn decrypt(&self, n: u64) -> u64 {
        let mut sum: u32 = TEA_DELTA << 4;
        let (mut x, mut y) = ((n >> 32) as u32, n as u32);
        let k0 = self.key[0];
        let k1 = self.key[1];
        let k2 = self.key[2];
        let k3 = self.key[3];

        for _ in 0..16 {
            y = y.wrapping_sub(k2.wrapping_add(x << 4) ^ x.wrapping_add(sum) ^ k3.wrapping_add(x >> 5));
            x = x.wrapping_sub(k0.wrapping_add(y << 4) ^ y.wrapping_add(sum) ^ k1.wrapping_add(y >> 5));
            sum = sum.wrapping_sub(TEA_DELTA);
        }

        ((x as u64) << 32) | y as u64
    }

    #[inline]
    pub fn new(key: &[u8; 16]) -> Self {
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

#[allow(dead_code)]
pub fn tea16_encrypt(text: &mut [u8; 8], key: &[u8; 16]) {
    let mut n = u64::from_be_bytes(*text);
    n = Tea16::new(key).encrypt(n);
    *text = n.to_be_bytes();
}

#[allow(dead_code)]
pub fn tea16_decrypt(text: &mut [u8; 8], key: &[u8; 16]) {
    let mut n = u64::from_be_bytes(*text);
    n = Tea16::new(key).decrypt(n);
    *text = n.to_be_bytes();
}
