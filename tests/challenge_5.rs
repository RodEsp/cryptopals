use cryptopals::ascii;
use cryptopals::hex;
use cryptopals::xor;

#[test]
fn encrypt_with_repeating_key_xor() {
    assert_eq!(
		hex::bytes_to_string(&xor::repeating_key(
			&ascii::string_to_bytes("ICE"),
			&ascii::string_to_bytes(
				"Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal"
			)
		)), "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f".to_uppercase()
	);
}
