use base64::{engine::general_purpose, Engine as _};
use crypto::buffer::{BufferResult, ReadBuffer, WriteBuffer};
use crypto::digest::Digest;
use crypto::md5::Md5;
use crypto::{
    aes::{cbc_decryptor, KeySize::KeySize256},
    blockmodes, buffer,
    symmetriccipher::SymmetricCipherError,
};

fn aes256_cbc_decrypt(encrypted_data: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, SymmetricCipherError> {
    let mut decryptor = cbc_decryptor(KeySize256, key, iv, blockmodes::PkcsPadding);

    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(encrypted_data);
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        let result = decryptor.decrypt(&mut read_buffer, &mut write_buffer, true)?;
        final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().copied());
        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => {}
        }
    }

    Ok(final_result)
}

pub fn decrypt_core(message: &[u8], key: &str) -> Vec<u8> {
    let mut md5er = Md5::new();
    let string = String::from(key);
    md5er.input_str(&string);
    let md5 = md5er.result_str();

    let md5_u8_array = md5.as_bytes();

    let aes_key: [u8; 32] = match md5_u8_array.try_into() {
        Ok(ok) => ok,
        Err(_error) => return String::from("decrypt error, code 1").into_bytes(),
    };

    let aes_iv: [u8; 16] = match (&aes_key[0..16]).try_into() {
        Ok(ok) => ok,
        Err(_error) => return String::from("decrypt error, code 1").into_bytes(),
    };

    // println!("message as_bytes {:?}", message.as_bytes());
    let message_decode_base64 = match general_purpose::STANDARD.decode(message) {
        Ok(ok) => ok,
        Err(err) => return err.to_string().into_bytes(),
    };
    // println!("base64_decode_result {:?}", base64_decode_result);

    let decrypted_result = aes256_cbc_decrypt(&message_decode_base64[..], &aes_key, &aes_iv);

    match decrypted_result {
        Ok(ok) => ok,
        Err(_error) => String::from("decrypt error, code 3").into_bytes(),
    }
}

pub fn decrypt_string(message: &str, key: &str) -> String {
    let message_bytes = message.as_bytes();
    let result_bytes = decrypt_core(message_bytes, key);
    if let Ok(ok) = String::from_utf8(result_bytes) {
        return ok;
    }
    String::from("decrypt error, code 4")
}

use std::fs::{self, write, File};
use std::io::Read;

pub fn decrypt_file(path: &str, key: &str, out_file: &str) -> Option<String> {
    if let Ok(mut f) = File::open(path) {
        if let Ok(metadata) = fs::metadata(path) {
            let mut buffer = vec![0; metadata.len() as usize];

            if f.read(&mut buffer).is_ok() {
                let result_bytes = decrypt_core(&buffer, key);

                if write(out_file, result_bytes).is_ok() {
                    return None;
                }
                return Some(String::from("write fail"));
            }
            return Some(String::from("buffer overflow"));
        }
        return Some(String::from("unable to read metadata"));
    }
    Some(String::from("no file found"))
}