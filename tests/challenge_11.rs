use cryptopals::block_ciphers::aes_128::encryption_oracle;

#[test]
fn detect_ecb_vs_cbc_encryption() {
    encryption_oracle::encrypt("This is a long text that repeats some long text because this is a long text that repeats some long text.");
}
