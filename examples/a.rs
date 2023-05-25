use cipher::split_decrypt_file;
use cipher::split_encrypt_file;

fn main() {
    split_encrypt_file("./icons/Square310x310Logo.png", "123", "./spilt");
    split_decrypt_file("./spilt", "123", "./spilt-d.jpg");
}
