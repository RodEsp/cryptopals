pub mod aes_128 {
    pub mod ecb {
        use openssl::symm::{Cipher, Mode};

        pub fn encrypt(key: &str, bytes: Vec<u8>, pad: bool) -> Vec<u8> {
            let mut encrypter = openssl::symm::Crypter::new(
                Cipher::aes_128_ecb(),
                Mode::Encrypt,
                key.as_bytes(),
                None,
            )
            .unwrap();
            encrypter.pad(pad);

            let mut encrypted_bytes = vec![0; bytes.len() + Cipher::aes_128_ecb().block_size()];
            let count = encrypter
                .update(bytes.as_slice(), &mut encrypted_bytes)
                .unwrap();

            let final_count = encrypter.finalize(&mut encrypted_bytes[count..]).unwrap();
            encrypted_bytes.truncate(count + final_count);

            encrypted_bytes
        }
        pub fn decrypt(key: &str, bytes: Vec<u8>, pad: bool) -> Vec<u8> {
            let mut decrypter = openssl::symm::Crypter::new(
                Cipher::aes_128_ecb(),
                Mode::Decrypt,
                key.as_bytes(),
                None,
            )
            .unwrap();
            decrypter.pad(pad);

            let mut decrypted_bytes = vec![0; bytes.len() + Cipher::aes_128_ecb().block_size()];
            let count = decrypter
                .update(bytes.as_slice(), &mut decrypted_bytes)
                .unwrap();

            let final_count = decrypter.finalize(&mut decrypted_bytes[count..]).unwrap();
            decrypted_bytes.truncate(count + final_count);

            decrypted_bytes
        }
    }

    pub mod cbc {
        use crate::{traits::Pkcs7Padding, xor};

        pub fn encrypt(key: &str, iv: Vec<u8>, mut bytes: Vec<u8>) -> Vec<u8> {
            let mut encrypted_bytes = Vec::<u8>::new();

            // Make bytes.len() a multiple of the block size for cbc encryption
            bytes.pad_pkcs7();

            let mut blocks = bytes.chunks(16);
            let mut prev_block = iv;

            while blocks.len() != 0 {
                let block = blocks.nth(0).unwrap().to_vec();
                let xor_ed_block = xor::vecs(&block, &prev_block);
                let mut encrypted_block = super::ecb::encrypt(key, xor_ed_block, false);
                prev_block = encrypted_block.clone();
                encrypted_bytes.append(&mut encrypted_block);
            }

            return encrypted_bytes;
        }

        pub fn decrypt(key: &str, iv: Vec<u8>, bytes: Vec<u8>) -> Vec<u8> {
            let mut decrypted_bytes = Vec::<u8>::new();

            let mut encrypted_blocks = bytes.chunks(16);
            let mut prev_block = iv;

            while encrypted_blocks.len() != 0 {
                let block = encrypted_blocks.nth(0).unwrap().to_vec();
                let decrypted_block = super::ecb::decrypt(key, block.clone(), false);
                let mut xor_ed_block = xor::vecs(&decrypted_block, &prev_block);
                prev_block = block;
                decrypted_bytes.append(&mut xor_ed_block);
            }

            return decrypted_bytes.remove_pkcs7_padding().to_vec();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{ascii, hex};

    use super::*;

    #[test]
    fn aes_128_ecb_with_less_than_16_bytes() {
        let plain_text = "this is a test";

        let encrypted_bytes =
            aes_128::ecb::encrypt("YELLOW SUBMARINE", ascii::string_to_bytes(plain_text), true);

        assert_eq!(
            hex::bytes_to_string(&encrypted_bytes),
            "9BC7F5B7ED2E6AB1C971A38ACEC089E7"
        );
    }

    #[test]
    fn aes_128_ecb_with_16_bytes() {
        let plain_text = "testing 16 bytes";

        let encrypted_bytes =
            aes_128::ecb::encrypt("YELLOW SUBMARINE", ascii::string_to_bytes(plain_text), true);

        assert_eq!(
            hex::bytes_to_string(&encrypted_bytes),
            "8425D90CC99E602FD6556ECFCAB2257460FA36707E45F499DBA0F25B922301A5"
        );
    }

    #[test]
    fn aes_128_ecb_with_more_than_16_bytes() {
        let plain_text =
            "this is a much longer test because there are more bytes in the plain text";
        let key = "YELLOW SUBMARINE";

        let encrypted_bytes = aes_128::ecb::encrypt(key, ascii::string_to_bytes(plain_text), true);

        let decrypted_bytes = aes_128::ecb::decrypt(key, encrypted_bytes, true);

        assert_eq!(decrypted_bytes, plain_text.as_bytes());
    }

    #[test]
    fn aes_128_ecb_with_32_bytes() {
        let plain_text = "testing 32 bytestesting 32 bytes";
        let key = "YELLOW SUBMARINE";

        let encrypted_bytes = aes_128::ecb::encrypt(key, ascii::string_to_bytes(plain_text), true);
        let decrypted_bytes = aes_128::ecb::decrypt(key, encrypted_bytes, true);

        assert_eq!(decrypted_bytes, plain_text.as_bytes());
    }

    #[test]
    fn aes_128_cbc_with_less_than_16_bytes() {
        let plain_text = "this is a test";
        let key = "YELLOW SUBMARINE";
        let iv = b"0000000000000000";

        let encrypted_bytes =
            aes_128::cbc::encrypt(key, iv.to_vec(), ascii::string_to_bytes(&plain_text));

        let decrypted_bytes = aes_128::cbc::decrypt(key, iv.to_vec(), encrypted_bytes.clone());

        assert_eq!(decrypted_bytes, plain_text.as_bytes());
    }

    #[test]
    fn aes_128_cbc_with_16_bytes() {
        let plain_text = "testing 16 bytes";
        let key = "YELLOW SUBMARINE";
        let iv = b"0000000000000000";

        let encrypted_bytes =
            aes_128::cbc::encrypt(key, iv.to_vec(), ascii::string_to_bytes(&plain_text));

        let decrypted_bytes = aes_128::cbc::decrypt(key, iv.to_vec(), encrypted_bytes.clone());

        assert_eq!(decrypted_bytes, plain_text.as_bytes());
    }

    #[test]
    fn aes_128_cbc_with_more_than_16_bytes() {
        let plain_text = "this is a test with more bytes";
        let key = "YELLOW SUBMARINE";
        let iv = b"0000000000000000";

        let encrypted_bytes =
            aes_128::cbc::encrypt(key, iv.to_vec(), ascii::string_to_bytes(&plain_text));

        let decrypted_bytes = aes_128::cbc::decrypt(key, iv.to_vec(), encrypted_bytes.clone());

        assert_eq!(decrypted_bytes, plain_text.as_bytes());
    }
}
