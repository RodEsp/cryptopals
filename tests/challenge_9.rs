use cryptopals::ascii;
use cryptopals::traits::Pad;

#[test]
fn pad_bytes_with_pkcs7() {
    let mut bytes = ascii::string_to_bytes("YELLOW SUBMARINE");

    assert_eq!(20, bytes.pad(4).len());
    assert!(bytes.ends_with(&[0x04, 0x04, 0x04, 0x04]));
}
