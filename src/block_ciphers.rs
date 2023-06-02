use openssl::symm::{decrypt, encrypt, Cipher};

pub fn aes_128_ecb_encrypt(key: &str, bytes: Vec<u8>) -> Vec<u8> {
    encrypt(
        Cipher::aes_128_ecb(),
        key.as_bytes(),
        None,
        bytes.as_slice(),
    )
    .expect("AES-128 ECB encryption Failed.")
}
pub fn aes_128_ecb_decrypt(key: &str, bytes: Vec<u8>) -> Vec<u8> {
    decrypt(
        Cipher::aes_128_ecb(),
        key.as_bytes(),
        None,
        bytes.as_slice(),
    )
    .expect("AES-128 ECB decryption Failed.")
}
