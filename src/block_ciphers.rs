const SUBSTITUTION_BOX: [u8; 256] = [
    0x63, 0x7c, 0x77, 0x7b, 0xf2, 0x6b, 0x6f, 0xc5, 0x30, 0x01, 0x67, 0x2b, 0xfe, 0xd7, 0xab, 0x76,
    0xca, 0x82, 0xc9, 0x7d, 0xfa, 0x59, 0x47, 0xf0, 0xad, 0xd4, 0xa2, 0xaf, 0x9c, 0xa4, 0x72, 0xc0,
    0xb7, 0xfd, 0x93, 0x26, 0x36, 0x3f, 0xf7, 0xcc, 0x34, 0xa5, 0xe5, 0xf1, 0x71, 0xd8, 0x31, 0x15,
    0x04, 0xc7, 0x23, 0xc3, 0x18, 0x96, 0x05, 0x9a, 0x07, 0x12, 0x80, 0xe2, 0xeb, 0x27, 0xb2, 0x75,
    0x09, 0x83, 0x2c, 0x1a, 0x1b, 0x6e, 0x5a, 0xa0, 0x52, 0x3b, 0xd6, 0xb3, 0x29, 0xe3, 0x2f, 0x84,
    0x53, 0xd1, 0x00, 0xed, 0x20, 0xfc, 0xb1, 0x5b, 0x6a, 0xcb, 0xbe, 0x39, 0x4a, 0x4c, 0x58, 0xcf,
    0xd0, 0xef, 0xaa, 0xfb, 0x43, 0x4d, 0x33, 0x85, 0x45, 0xf9, 0x02, 0x7f, 0x50, 0x3c, 0x9f, 0xa8,
    0x51, 0xa3, 0x40, 0x8f, 0x92, 0x9d, 0x38, 0xf5, 0xbc, 0xb6, 0xda, 0x21, 0x10, 0xff, 0xf3, 0xd2,
    0xcd, 0x0c, 0x13, 0xec, 0x5f, 0x97, 0x44, 0x17, 0xc4, 0xa7, 0x7e, 0x3d, 0x64, 0x5d, 0x19, 0x73,
    0x60, 0x81, 0x4f, 0xdc, 0x22, 0x2a, 0x90, 0x88, 0x46, 0xee, 0xb8, 0x14, 0xde, 0x5e, 0x0b, 0xdb,
    0xe0, 0x32, 0x3a, 0x0a, 0x49, 0x06, 0x24, 0x5c, 0xc2, 0xd3, 0xac, 0x62, 0x91, 0x95, 0xe4, 0x79,
    0xe7, 0xc8, 0x37, 0x6d, 0x8d, 0xd5, 0x4e, 0xa9, 0x6c, 0x56, 0xf4, 0xea, 0x65, 0x7a, 0xae, 0x08,
    0xba, 0x78, 0x25, 0x2e, 0x1c, 0xa6, 0xb4, 0xc6, 0xe8, 0xdd, 0x74, 0x1f, 0x4b, 0xbd, 0x8b, 0x8a,
    0x70, 0x3e, 0xb5, 0x66, 0x48, 0x03, 0xf6, 0x0e, 0x61, 0x35, 0x57, 0xb9, 0x86, 0xc1, 0x1d, 0x9e,
    0xe1, 0xf8, 0x98, 0x11, 0x69, 0xd9, 0x8e, 0x94, 0x9b, 0x1e, 0x87, 0xe9, 0xce, 0x55, 0x28, 0xdf,
    0x8c, 0xa1, 0x89, 0x0d, 0xbf, 0xe6, 0x42, 0x68, 0x41, 0x99, 0x2d, 0x0f, 0xb0, 0x54, 0xbb, 0x16,
];

const INV_SUBSTITUTION_BOX: [u8; 256] = [
    0x52, 0x09, 0x6a, 0xd5, 0x30, 0x36, 0xa5, 0x38, 0xbf, 0x40, 0xa3, 0x9e, 0x81, 0xf3, 0xd7, 0xfb,
    0x7c, 0xe3, 0x39, 0x82, 0x9b, 0x2f, 0xff, 0x87, 0x34, 0x8e, 0x43, 0x44, 0xc4, 0xde, 0xe9, 0xcb,
    0x54, 0x7b, 0x94, 0x32, 0xa6, 0xc2, 0x23, 0x3d, 0xee, 0x4c, 0x95, 0x0b, 0x42, 0xfa, 0xc3, 0x4e,
    0x08, 0x2e, 0xa1, 0x66, 0x28, 0xd9, 0x24, 0xb2, 0x76, 0x5b, 0xa2, 0x49, 0x6d, 0x8b, 0xd1, 0x25,
    0x72, 0xf8, 0xf6, 0x64, 0x86, 0x68, 0x98, 0x16, 0xd4, 0xa4, 0x5c, 0xcc, 0x5d, 0x65, 0xb6, 0x92,
    0x6c, 0x70, 0x48, 0x50, 0xfd, 0xed, 0xb9, 0xda, 0x5e, 0x15, 0x46, 0x57, 0xa7, 0x8d, 0x9d, 0x84,
    0x90, 0xd8, 0xab, 0x00, 0x8c, 0xbc, 0xd3, 0x0a, 0xf7, 0xe4, 0x58, 0x05, 0xb8, 0xb3, 0x45, 0x06,
    0xd0, 0x2c, 0x1e, 0x8f, 0xca, 0x3f, 0x0f, 0x02, 0xc1, 0xaf, 0xbd, 0x03, 0x01, 0x13, 0x8a, 0x6b,
    0x3a, 0x91, 0x11, 0x41, 0x4f, 0x67, 0xdc, 0xea, 0x97, 0xf2, 0xcf, 0xce, 0xf0, 0xb4, 0xe6, 0x73,
    0x96, 0xac, 0x74, 0x22, 0xe7, 0xad, 0x35, 0x85, 0xe2, 0xf9, 0x37, 0xe8, 0x1c, 0x75, 0xdf, 0x6e,
    0x47, 0xf1, 0x1a, 0x71, 0x1d, 0x29, 0xc5, 0x89, 0x6f, 0xb7, 0x62, 0x0e, 0xaa, 0x18, 0xbe, 0x1b,
    0xfc, 0x56, 0x3e, 0x4b, 0xc6, 0xd2, 0x79, 0x20, 0x9a, 0xdb, 0xc0, 0xfe, 0x78, 0xcd, 0x5a, 0xf4,
    0x1f, 0xdd, 0xa8, 0x33, 0x88, 0x07, 0xc7, 0x31, 0xb1, 0x12, 0x10, 0x59, 0x27, 0x80, 0xec, 0x5f,
    0x60, 0x51, 0x7f, 0xa9, 0x19, 0xb5, 0x4a, 0x0d, 0x2d, 0xe5, 0x7a, 0x9f, 0x93, 0xc9, 0x9c, 0xef,
    0xa0, 0xe0, 0x3b, 0x4d, 0xae, 0x2a, 0xf5, 0xb0, 0xc8, 0xeb, 0xbb, 0x3c, 0x83, 0x53, 0x99, 0x61,
    0x17, 0x2b, 0x04, 0x7e, 0xba, 0x77, 0xd6, 0x26, 0xe1, 0x69, 0x14, 0x63, 0x55, 0x21, 0x0c, 0x7d,
];

fn substitute_bytes(matrix: &mut [[u8; 4]; 4]) -> () {
    for (i, row) in matrix.clone().into_iter().enumerate() {
        for (y, byte) in row.into_iter().enumerate() {
            matrix[i][y] = SUBSTITUTION_BOX[byte as usize];
        }
    }
}

fn shift_rows(matrix: &mut [[u8; 4]; 4]) -> () {
    let mut original_matrix = matrix.clone();

    for (i, row) in matrix.into_iter().enumerate() {
        match i {
            1 => {
                for (y, mut byte) in row.into_iter().enumerate() {
                    if y == 0 {
                        byte = &mut original_matrix[i][y + 3];
                    } else {
                        byte = &mut original_matrix[i][y - 1]
                    }
                }
            }
            2 => {
                for (y, mut byte) in row.into_iter().enumerate() {
                    if y == 0 || y == 1 {
                        byte = &mut original_matrix[i][y + 2];
                    } else {
                        byte = &mut original_matrix[i][y - 2]
                    }
                }
            }
            3 => {
                for (y, mut byte) in row.into_iter().enumerate() {
                    if y == 3 {
                        byte = &mut original_matrix[i][y - 3];
                    } else {
                        byte = &mut original_matrix[i][y + 1];
                    }
                }
            }
            _ => (),
        }
    }
}

pub mod aes_128 {
    use super::*;

    pub fn encrypt(key: &str, bytes: Vec<u8>) -> Vec<u8> {
        assert_eq!(bytes.len(), 16); // bytes should always have length 16 because AES-128 only works on block sizes of 16 bytes (128 bits)

        let encrypted_bytes = vec![];

        // Key Expansion

        // Organize bytes into AES state matrix
        let mut state_matrix: [[u8; 4]; 4] = [
            [bytes[0], bytes[4], bytes[8], bytes[12]],
            [bytes[1], bytes[5], bytes[9], bytes[13]],
            [bytes[2], bytes[6], bytes[10], bytes[14]],
            [bytes[3], bytes[7], bytes[11], bytes[15]],
        ];

        // Initial round key addition

        // Encryption rounds
        let mut round = 1;
        while round <= 9 {
            // for AES 128 you use 10 rounds.
            // Substitute Bytes
            substitute_bytes(&mut state_matrix);

            // Shift Rows

            // Mix Columns
            if round != 10 { // doesn't happen on last round
            }

            // Add round key

            round += 1;
        }

        //Final round
        // Substitute Bytes

        // Shift Rows

        // Add final round key

        // Add result of encryption rounds to encrypted_bytes

        return encrypted_bytes;
    }

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
            bytes.add_pkcs7_padding();

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
