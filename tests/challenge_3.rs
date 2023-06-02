use cryptopals::hex;
use cryptopals::utilities::find_single_char_repeating_xor_key_and_message;

#[test]
fn find_key_for_repeating_single_char_xor() {
    let (key, message, ..) = find_single_char_repeating_xor_key_and_message(&hex::string_to_bytes(
        "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736",
    ));

    assert_eq!(key.unwrap_or_default(), 'X');
    assert_eq!(
        message.unwrap_or_default(),
        "Cooking MC's like a pound of bacon"
    );
}
