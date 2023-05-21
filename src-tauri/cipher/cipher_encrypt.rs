use base64::{engine::general_purpose, Engine as _};
use crypto::buffer::{BufferResult, ReadBuffer, WriteBuffer};
use crypto::digest::Digest;
use crypto::md5::Md5;
use crypto::{
    aes::{cbc_encryptor, KeySize::KeySize256},
    blockmodes, buffer,
    symmetriccipher::SymmetricCipherError,
};

fn aes256_cbc_encrypt(data: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, SymmetricCipherError> {
    let mut encryptor = cbc_encryptor(KeySize256, key, iv, blockmodes::PkcsPadding);

    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(data);
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        let result = encryptor.encrypt(&mut read_buffer, &mut write_buffer, true)?;

        final_result.extend(
            write_buffer
                .take_read_buffer()
                .take_remaining()
                .iter()
                .map(|&i| i),
        );

        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => {}
        }
    }

    Ok(final_result)
}

pub fn encrypt_core(message: &[u8], key: &str) -> Vec<u8> {
    let mut md5er = Md5::new();
    let string = String::from(key);
    md5er.input_str(&string);
    let md5 = md5er.result_str();

    let md5_u8_array = md5.as_bytes();

    let aes_key: [u8; 32] = match md5_u8_array.try_into() {
        Ok(ok) => ok,
        Err(_error) => return String::from("decrypt encrypt, code 1").into_bytes(),
    };

    let aes_iv: [u8; 16] = match (&aes_key[0..16]).try_into() {
        Ok(ok) => ok,
        Err(_error) => return String::from("decrypt encrypt, code 2").into_bytes(),
    };

    let encrypted_result = aes256_cbc_encrypt(message, &aes_key, &aes_iv);

    let encrypted_data = match encrypted_result {
        Ok(ok) => ok,
        Err(_error) => return String::from("decrypt error, code 3").into_bytes(),
    };

    let mut ciphertext_base64_bytes = Vec::new();
    // make sure we'll have a slice big enough for base64 + padding
    ciphertext_base64_bytes.resize(encrypted_data.len() * 4 / 3 + 4, 0);

    let bytes_written = general_purpose::STANDARD
        .encode_slice(encrypted_data, &mut ciphertext_base64_bytes)
        .unwrap();

    // shorten our vec down to just what was written
    ciphertext_base64_bytes.truncate(bytes_written);

    return ciphertext_base64_bytes;
}

pub fn encrypt_string(message: &str, key: &str) -> String {
    let message_bytes = message.as_bytes();
    let result_bytes = encrypt_core(message_bytes, key);
    if let Ok(ok) = String::from_utf8(result_bytes) {
        return ok;
    }
    return String::from("Invalid UTF8");
}

use std::fs::{self, write, File};
use std::io::Read;

pub fn encrypt_file(path: &str, key: &str, out_file: &str) -> Option<String> {
    if let Ok(mut f) = File::open(&path) {
        if let Ok(metadata) = fs::metadata(&path) {
            let mut buffer = vec![0; metadata.len() as usize];

            if let Ok(_) = f.read(&mut buffer) {
                let result_bytes = encrypt_core(&buffer, key);

                if let Ok(_) = write(out_file, &result_bytes) {
                    return None;
                }
                return Some(String::from("write fail"));
            }
            return Some(String::from("buffer overflow"));
        }
        return Some(String::from("unable to read metadata"));
    }
    return Some(String::from("no file found"));
}
