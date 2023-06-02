use cryptopals::hex;
use cryptopals::xor;

#[test]
fn xor_hex_strings() {
    assert_eq!(
        hex::bytes_to_string(&xor::vecs(
            &hex::string_to_bytes("686974207468652062756c6c277320657965"),
            &hex::string_to_bytes("1c0111001f010100061a024b53535009181c")
        )),
        "746865206b696420646f6e277420706c6179".to_uppercase()
    );
}
