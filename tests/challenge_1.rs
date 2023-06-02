use cryptopals::base64;
use cryptopals::hex;

#[test]
fn hex_to_base64() {
    assert_eq!(base64::bytes_to_string(
        &hex::string_to_bytes("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d")
    ), 
    "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
}
