use byteorder::{BigEndian, ByteOrder};
use rand::{thread_rng, RngCore};

use crate::tea::{GenericArray, Tea16};

pub fn qqtea_encrypt(text: &[u8], key: &[u8]) -> Vec<u8> {
    let fill_count = 9 - (text.len() + 1) % 8;

    let mut plaintext = vec![0u8; 1 + fill_count + text.len() + 7];
    let plaintext_len = plaintext.len();

    plaintext[0] = (fill_count as u8 - 2) | 0xF8;
    if cfg!(debug_assertions) {
        //这里是为了和pytea对拍，填充220
        plaintext[1..fill_count + 1].fill(220);
    } else {
        thread_rng().fill_bytes(&mut plaintext[1..fill_count + 1]);
    }
    plaintext[fill_count + 1..plaintext_len - 7].copy_from_slice(text);

    let mut work_block: Vec<u64> = vec![0; plaintext.len() / 8];

    BigEndian::read_u64_into(&plaintext, &mut work_block);

    let mut iv1 = 0u64;
    let mut iv2 = 0u64;
    let mut holder: u64;

    let cipher = Tea16::new(GenericArray::from_slice(key));

    for block in work_block.iter_mut() {
        holder = *block ^ iv1;

        iv1 = cipher.encrypt(holder);

        iv1 ^= iv2;

        iv2 = holder;

        *block = iv1;
    }

    BigEndian::write_u64_into(&work_block, &mut plaintext);

    plaintext
}

pub fn qqtea_decrypt(text: &[u8], key: &[u8]) -> Vec<u8> {
    let mut work_block: Vec<u64> = vec![0; text.len() / 8];

    BigEndian::read_u64_into(text, &mut work_block);

    let mut iv1 = 0u64;
    let mut iv2 = 0u64;
    let mut holder: u64;
    let mut tmp_block: u64;

    let cipher = Tea16::new(GenericArray::from_slice(key));

    for block in work_block.iter_mut() {
        tmp_block = *block ^ iv2;

        tmp_block = cipher.decrypt(tmp_block);

        iv2 = tmp_block;

        holder = tmp_block ^ iv1;

        iv1 = *block;

        *block = holder;
    }

    let mut result = vec![0u8; text.len()];

    BigEndian::write_u64_into(&work_block, &mut result);

    let begin_pos = ((result[0] as usize) & 7) + 3;
    let end_pos = result.len() - 7;

    result[begin_pos..end_pos].to_owned()
}
