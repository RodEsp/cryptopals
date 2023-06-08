use cryptopals::ascii;
use cryptopals::traits::Pkcs7Padding;

#[test]
fn pad_16_bytes_with_pkcs7() {
    let mut bytes = ascii::string_to_bytes("YELLOW SUBMARINE");

    assert_eq!(32, bytes.pad_pkcs7().len());
    assert!(bytes.ends_with(&[
        0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10,
        0x10
    ]));
}

#[test]
fn pad_14_bytes_with_pkcs7() {
    let mut bytes = ascii::string_to_bytes("this is a test");

    assert_eq!(16, bytes.pad_pkcs7().len());
    assert!(bytes.ends_with(&[0x02, 0x02]));
}

#[test]
fn pad_1_byte_with_pkcs7() {
    let mut bytes = ascii::string_to_bytes("1");

    assert_eq!(16, bytes.pad_pkcs7().len());
    assert!(bytes.ends_with(&[
        0x0F, 0x0F, 0x0F, 0x0F, 0x0F, 0x0F, 0x0F, 0x0F, 0x0F, 0x0F, 0x0F, 0x0F, 0x0F, 0x0F, 0x0F
    ]));
}
