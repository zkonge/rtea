use std::{
    error::Error,
    fmt::{self, Display, Formatter},
    ops::Range,
};

use rand::{rng, RngCore};

use crate::tea::Tea16;

#[derive(Debug)]
pub enum BlockError {
    BadCipherText,
    InsufficientOutputBuffer,
}

impl Display for BlockError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            BlockError::BadCipherText => "Bad cipher text",
            BlockError::InsufficientOutputBuffer => "Insufficient output buffer",
        })
    }
}

impl Error for BlockError {}

#[derive(Clone, Copy)]
pub struct QQTea(Tea16);

impl QQTea {
    pub fn new(key: [u8; 16]) -> Self {
        Self(Tea16::new(key))
    }

    #[inline(always)]
    fn estimate_ciphertext_padding_length(plaintext_length: usize) -> usize {
        9 - (plaintext_length + 1) % 8
    }

    #[inline(always)]
    pub fn estimate_ciphertext_size(plaintext_length: usize) -> usize {
        let fill_count = Self::estimate_ciphertext_padding_length(plaintext_length);
        1 + fill_count + plaintext_length + 7
    }

    pub fn encrypt_inout(&self, data: &[u8], out: &mut [u8]) -> Result<(), BlockError> {
        let out = out
            .get_mut(..Self::estimate_ciphertext_size(data.len()))
            .ok_or(BlockError::InsufficientOutputBuffer)?;
        let fill_count = Self::estimate_ciphertext_padding_length(data.len());

        out[0] = (fill_count as u8 - 2) | 0xF8;
        if cfg!(debug_assertions) {
            // test with standard pytea, filling with 220
            out[1..fill_count + 1].fill(220);
        } else {
            rng().fill_bytes(&mut out[1..fill_count + 1]);
        }

        // make borrow checker happy
        let out_len = out.len();
        out[fill_count + 1..out_len - 7].copy_from_slice(data);

        let mut iv1 = 0u64;
        let mut iv2 = 0u64;
        let mut holder;

        let cipher = self.0;

        for block in out.chunks_exact_mut(8) {
            let block: &mut [u8; 8] = block.try_into().unwrap();

            holder = u64::from_be_bytes(*block) ^ iv1;

            iv1 = cipher.encrypt(holder);
            iv1 ^= iv2;
            iv2 = holder;

            *block = iv1.to_be_bytes();
        }

        Ok(())
    }

    pub fn decrypt_inout(&self, data: &mut [u8]) -> Result<Range<usize>, BlockError> {
        if data.len() < 16 || data.len() % 8 != 0 {
            return Err(BlockError::BadCipherText);
        }

        let mut iv1;
        let mut iv2 = 0u64;
        let mut holder = 0u64;

        let cipher = self.0;

        for block in data.chunks_exact_mut(8) {
            let block: &mut [u8; 8] = block.try_into().unwrap();

            iv1 = u64::from_be_bytes(*block);

            iv2 ^= iv1;
            iv2 = cipher.decrypt(iv2);
            *block = (iv2 ^ holder).to_be_bytes();
            holder = iv1;
        }

        let begin_pos = ((data[0] as usize) & 7) + 3;
        let end_pos = data.len() - 7;

        data.get(begin_pos..end_pos)
            .ok_or(BlockError::BadCipherText)?;

        Ok(begin_pos..end_pos)
    }
}

pub fn qqtea_encrypt(data: &[u8], key: [u8; 16]) -> Vec<u8> {
    let mut out = vec![0u8; QQTea::estimate_ciphertext_size(data.len())];
    let cipher = QQTea::new(key);
    cipher.encrypt_inout(data, &mut out).unwrap();
    out
}

pub fn qqtea_decrypt(data: &[u8], key: [u8; 16]) -> Vec<u8> {
    let mut out = data.to_vec();
    let cipher = QQTea::new(key);
    let range = cipher.decrypt_inout(&mut out).unwrap();
    out[range].to_vec()
}
