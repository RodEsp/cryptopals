use cryptopals::ascii;
use cryptopals::traits::Pad;

#[test]
fn pad_bytes_with_pkcs7() {
    let mut bytes = ascii::string_to_bytes("YELLOW SUBMARINE");

    assert_eq!(32, bytes.pad_pkcs7().len());
    assert!(bytes.ends_with(&[
        0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10,
        0x10
    ]));

    let mut bytes = ascii::string_to_bytes("this is a test");

    assert_eq!(16, bytes.pad_pkcs7().len());
    assert!(bytes.ends_with(&[0x02, 0x02]));

    let mut bytes = ascii::string_to_bytes("1");

    assert_eq!(16, bytes.pad_pkcs7().len());
    assert!(bytes.ends_with(&[
        0x0F, 0x0F, 0x0F, 0x0F, 0x0F, 0x0F, 0x0F, 0x0F, 0x0F, 0x0F, 0x0F, 0x0F, 0x0F, 0x0F, 0x0F
    ]));
}
