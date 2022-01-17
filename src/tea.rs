use byteorder::{BigEndian, ByteOrder};
pub use generic_array::{
    typenum::{U16, U8},
    GenericArray,
};

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
    pub fn new(key: &GenericArray<u8, U16>) -> Self {
        Self {
            key: [
                BigEndian::read_u32(&key[0..4]),
                BigEndian::read_u32(&key[4..8]),
                BigEndian::read_u32(&key[8..12]),
                BigEndian::read_u32(&key[12..16]),
            ],
        }
    }
}

pub fn tea16_encrypt(text: &mut [u8], key: &[u8]) {
    let key: &GenericArray<u8, U16> = GenericArray::from_slice(key);

    let mut n = BigEndian::read_u64(text);

    n = Tea16::new(key).encrypt(n);

    BigEndian::write_u64(text, n);
}

pub fn tea16_decrypt(text: &mut [u8], key: &[u8]) {
    let key: &GenericArray<u8, U16> = GenericArray::from_slice(key);

    let mut n = BigEndian::read_u64(text);

    n = Tea16::new(key).decrypt(n);

    BigEndian::write_u64(text, n);
}
