use std::{
    fs::File,
    io::{BufRead, BufReader},
};

mod ascii;
mod base64;
mod hex;
mod log;
mod xor;

fn main() {
    // Challenge 1
    println!("Challenge 1: {}", base64::encode(&hex::decode("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d")));

    // Challenge 2
    println!(
        "Challenge 2: {}",
        hex::encode(&xor::vecs(
            &hex::decode("686974207468652062756c6c277320657965"),
            &hex::decode("1c0111001f010100061a024b53535009181c")
        ))
    );

    // Challenge 3
    let (key, message, ..) = find_encryption_key_and_message(&hex::decode(
        "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736",
    ));

    println!(
        "Challenge 3: Key = '{}', Message = \"{}\"",
        key.unwrap_or_default(),
        message.unwrap_or_default()
    );

    // Challenge 4
    let file =
        File::open("./data/challenge4.txt").expect("Was not able to read ./data/challenge4.txt");
    let reader = BufReader::new(file);

    let mut highest_scoring_key_and_message: (Option<char>, Option<String>) = (None, None);
    let mut highest_score: usize = 0;
    for line in reader.lines() {
        let (key, message, score) =
            find_encryption_key_and_message(&hex::decode(&line.unwrap_or_default()));

        if score > highest_score {
            highest_scoring_key_and_message = (key, message);
            highest_score = score;
        }
    }

    println!(
        "Challenge 4: Key = '{}', Message = \"{}\"",
        highest_scoring_key_and_message.0.unwrap_or_default(),
        highest_scoring_key_and_message.1.unwrap_or_default()
    );
}

// Checks how many times the 13 most common english language characters appear in a string (represented by its individual bytes) and returns their count
fn english_score(vec: &Vec<u8>) -> usize {
    let mut score = 0;

    // TODO: These are ordered from most common to least common so we could give them a weight for the final score but for now we are just straight up counting them.
    let most_common_letters = ['e', 't', 'a', 'o', 'i', ' ', 's', 'h', 'r', 'd', 'l', 'u'];
    for byte in vec {
        if most_common_letters.contains(&(*byte as char)) {
            score += 1;
        }
    }

    return score;
}

fn find_encryption_key_and_message(bytes: &Vec<u8>) -> (Option<char>, Option<String>, usize) {
    // Create a vec of tuples that contain the character, message, and a score
    // The score notes the likelyhood that the message is the secret message
    // Which also means taht the character is our secret key
    let scores = ascii::alphabet()
        .into_iter()
        .map(move |char| {
            // XOR the given hex_string against each ASCII character
            let decrypted_bytes = xor::vec_against_char(bytes, char);

            // Take the resulting bytes and figure out which one is the most likely to be a secret message, written in English
            if decrypted_bytes
                .clone()
                .into_iter()
                // Filter for strings that only contain chars that exist in the english language, including newline chars
                .all(|byte| matches!(byte as char, ' '..='}' | '\r' | '\n'))
            {
                // Give a score to each secret key and message that states how likely it is to be the secret info we're after
                (
                    char,
                    ascii::encode(&decrypted_bytes),
                    english_score(&decrypted_bytes),
                )
            } else {
                (char, ascii::encode(&decrypted_bytes), 0)
            }
        })
        .collect::<Vec<_>>();

    // Get the most likely key and message based on the max score for each one
    let mut most_likely_key_and_message: (Option<char>, Option<String>, usize) = (None, None, 0);
    scores.into_iter().for_each(|item| {
        if item.2 > most_likely_key_and_message.2 {
            most_likely_key_and_message = ((item.0).into(), (item.1).into(), item.2);
        }
    });

    return most_likely_key_and_message;
}
