use rand::{thread_rng, RngCore};

use crate::tea::Tea16;

pub fn qqtea_encrypt(text: &[u8], key: &[u8; 16]) -> Vec<u8> {
    let fill_count = 9 - (text.len() + 1) % 8;

    let mut plaintext = vec![0u8; 1 + fill_count + text.len() + 7];

    plaintext[0] = (fill_count as u8 - 2) | 0xF8;
    if cfg!(debug_assertions) {
        //这里是为了和pytea对拍，填充220
        plaintext[1..fill_count + 1].fill(220);
    } else {
        thread_rng().fill_bytes(&mut plaintext[1..fill_count + 1]);
    }

    // Make borrow checker happy
    let plaintext_len = plaintext.len();
    plaintext[fill_count + 1..plaintext_len - 7].copy_from_slice(text);

    let mut iv1 = 0u64;
    let mut iv2 = 0u64;
    let mut holder: u64;

    let cipher = Tea16::new(key);

    for block in plaintext.chunks_exact_mut(8) {
        let block: &mut [u8; 8] = block.try_into().unwrap();

        holder = u64::from_be_bytes(*block) ^ iv1;
    
        iv1 = cipher.encrypt(holder);
        iv1 ^= iv2;
        iv2 = holder;

        *block = iv1.to_be_bytes();
    }

    plaintext
}

pub fn qqtea_decrypt(text: &[u8], key: &[u8; 16]) -> Vec<u8> {
    let mut result = text.to_vec();

    let mut iv1 = 0u64;
    let mut iv2 = 0u64;
    let mut holder: u64;
    let mut tmp_block: u64;

    let cipher = Tea16::new(key);

    for block in result.chunks_exact_mut(8) {
        let block: &mut [u8; 8] = block.try_into().unwrap();

        let n = u64::from_be_bytes(*block);

        tmp_block = n ^ iv2;
        tmp_block = cipher.decrypt(tmp_block);
        iv2 = tmp_block;
        holder = tmp_block ^ iv1;
        iv1 = n;

        *block = holder.to_be_bytes();
    }

    let begin_pos = ((result[0] as usize) & 7) + 3;
    let end_pos = result.len() - 7;

    result[begin_pos..end_pos].to_owned()
}
