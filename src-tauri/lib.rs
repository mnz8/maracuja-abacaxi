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

// 字节数
// 最好保持 16 的整数倍；因为 base64 存在算法中，所以即使保持 16 的整数倍，也是会增加加密后的数据长度。
//  1 M
pub const BYTE_BLOCK_SIZE: usize = 1024 * 1024;
