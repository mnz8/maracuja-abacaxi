#[path = "cipher/cipher_decrypt.rs"]
pub mod cipher_decrypt;

#[path = "cipher/cipher_encrypt.rs"]
pub mod cipher_encrypt;

pub use cipher_decrypt::decrypt_core;
pub use cipher_decrypt::decrypt_file;
pub use cipher_decrypt::decrypt_string;
pub use cipher_decrypt::split_decrypt_file;
pub use cipher_encrypt::encrypt_core;
pub use cipher_encrypt::encrypt_file;
pub use cipher_encrypt::encrypt_string;
pub use cipher_encrypt::split_encrypt_file;

#[path = "tool.rs"]
pub mod tool;

pub use tool::get_now;

// 字节数 1 M
pub const BYTE_BLOCK_SIZE: usize = 1024 * 1024;
