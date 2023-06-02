pub mod aes_128 {
    pub mod ecb {
        pub fn encrypt(key: &str, bytes: Vec<u8>) -> Vec<u8> {
            openssl::symm::encrypt(
                openssl::symm::Cipher::aes_128_ecb(),
                key.as_bytes(),
                None,
                bytes.as_slice(),
            )
            .expect("AES-128 ECB encryption Failed.")
        }
        pub fn decrypt(key: &str, bytes: Vec<u8>) -> Vec<u8> {
            openssl::symm::decrypt(
                openssl::symm::Cipher::aes_128_ecb(),
                key.as_bytes(),
                None,
                bytes.as_slice(),
            )
            .expect("AES-128 ECB decryption Failed.")
        }
    }
}
