use cryptopals::block_ciphers::{aes_128_ecb_decrypt, aes_128_ecb_encrypt};
use cryptopals::utilities::read_file;
use cryptopals::{ascii, base64};

#[test]
fn decrypt_aes_128_ecb_with_given_key() {
    let plain_text = "I'm a little tea pot short and stout. Here is my handle. Here is my spout.";

    let encrypted_bytes =
        aes_128_ecb_encrypt("YELLOW SUBMARINE", ascii::string_to_bytes(&plain_text));

    let decrypted_bytes = aes_128_ecb_decrypt("YELLOW SUBMARINE", encrypted_bytes);

    assert_eq!(ascii::bytes_to_string(&decrypted_bytes), plain_text);
}

fn implement_cbc() {
    let base64_string = read_file("./data/challenge10.txt");
    let encrypted_bytes = base64::string_to_bytes(&base64_string);

    todo!();
}