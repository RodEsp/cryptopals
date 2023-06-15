use cryptopals::{block_ciphers::aes_128::encryption_oracle, utilities::read_file};

#[test]
fn detect_ecb_vs_cbc_encryption() {
    let text = read_file("./data/huckleberry finn, first sixth.txt");
    let (encryption_type, encrypted_bytes) = encryption_oracle::encrypt(&text);

    assert_eq!(
        encryption_type,
        encryption_oracle::detect_ecb_vs_cbc_encryption(encrypted_bytes)
    );
}
