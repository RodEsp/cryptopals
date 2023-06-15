use std::io::BufRead;

use cryptopals::block_ciphers::aes_128::ecb::detect;
use cryptopals::hex;
use cryptopals::utilities::line_buffer;

#[test]
fn detect_aes_128_ecb_encryption() {
    let file = line_buffer("./data/challenge8.txt");

    let mut lines_encrypted_with_ecb = Vec::<(usize, String)>::new();

    for (index, hex_string) in file.lines().enumerate() {
        let line_number = index + 1;
        let encrypted_bytes = hex::string_to_bytes(
            &hex_string
                .as_ref()
                .expect("Could not read hex string from file."),
        );

        if detect(encrypted_bytes) {
            lines_encrypted_with_ecb.push((line_number, hex_string.unwrap()))
        }
    }

    assert_eq!(lines_encrypted_with_ecb.len(), 1);
    assert_eq!(lines_encrypted_with_ecb[0].0, 133);
    assert_eq!(lines_encrypted_with_ecb[0].1, "d880619740a8a19b7840a8a31c810a3d08649af70dc06f4fd5d2d69c744cd283e2dd052f6b641dbf9d11b0348542bb5708649af70dc06f4fd5d2d69c744cd2839475c9dfdbc1d46597949d9c7e82bf5a08649af70dc06f4fd5d2d69c744cd28397a93eab8d6aecd566489154789a6b0308649af70dc06f4fd5d2d69c744cd283d403180c98c8f6db1f2a3f9c4040deb0ab51b29933f2c123c58386b06fba186a");
}
