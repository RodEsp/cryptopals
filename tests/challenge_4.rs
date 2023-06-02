use std::io::BufRead;

use cryptopals::hex;
use cryptopals::utilities::{find_single_char_repeating_xor_key_and_message, line_buffer};

#[test]
fn find_and_decrypt_line_encrypted_with_repeating_single_char_xor() {
    let line_buffer = line_buffer("./data/challenge4.txt");

    let mut highest_scoring_key = None;
    let mut highest_scoring_message = None;
    let mut highest_score: usize = 0;
    for line in line_buffer.lines() {
        let (key, message, score) = find_single_char_repeating_xor_key_and_message(
            &hex::string_to_bytes(&line.unwrap_or_default()),
        );

        if score > highest_score {
            highest_scoring_key = key;
            highest_scoring_message = message;
            highest_score = score;
        }
    }

    assert_eq!(highest_scoring_key.unwrap_or_default(), '5');
    assert_eq!(
        highest_scoring_message.unwrap_or_default(),
        "Now that the party is jumping\n"
    );
}
