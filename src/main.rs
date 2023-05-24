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
    println!("Challenge 1: {}", base64::bytes_to_string(&hex::string_to_bytes("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d")));

    // Challenge 2
    println!(
        "Challenge 2: {}",
        hex::bytes_to_string(&xor::vecs(
            &hex::string_to_bytes("686974207468652062756c6c277320657965"),
            &hex::string_to_bytes("1c0111001f010100061a024b53535009181c")
        ))
    );

    // Challenge 3
    let (key, message, ..) = find_encryption_key_and_message(&hex::string_to_bytes(
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

    let mut highest_scoring_key = None;
    let mut highest_scoring_message = None;
    let mut highest_score: usize = 0;
    for line in reader.lines() {
        let (key, message, score) =
            find_encryption_key_and_message(&hex::string_to_bytes(&line.unwrap_or_default()));

        if score > highest_score {
            highest_scoring_key = key;
            highest_scoring_message = message;
            highest_score = score;
        }
    }

    println!(
        "Challenge 4: Key = '{}', Message = \"{}\"",
        highest_scoring_key.unwrap_or_default(),
        highest_scoring_message.unwrap_or_default()
    );

    // Challenge 5
    println!(
        "Challenge 5: {}",
        hex::bytes_to_string(&xor::repeating_key(
            &ascii::string_to_bytes("ICE"),
            &ascii::string_to_bytes(
                "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal"
            )
        ))
    );

    // Challenge 6
    let file =
        File::open("./data/challenge6.txt").expect("Was not able to read ./data/challenge6.txt");
    let reader = BufReader::new(file);
    let base64_string: String = reader
        .lines()
        // Remove newlines from file
        .map(|line| {
            let mut s = line.unwrap_or_default();
            let len = s.trim_end_matches(&['\r', '\n'][..]).len();
            s.truncate(len);
            return s;
        })
        .collect();

    let bytes = base64::string_to_bytes(&base64_string);

    println!("{}", ascii::bytes_to_string(&bytes).unwrap_or_default());

    let min_keysize = 1;
    let max_keysize = 100;
    let mut keysize = 1;
}

fn hamming_distance(s1: &str, s2: &str) -> u32 {
    let mut hamming_distance = 0;

    let bytes1;
    let bytes2;
    if s1.len() > s2.len() {
        bytes1 = ascii::string_to_bytes(s1);
        bytes2 = ascii::string_to_bytes(s2);
    } else {
        bytes2 = ascii::string_to_bytes(s1);
        bytes1 = ascii::string_to_bytes(s2);
    }

    for i in 0..bytes1.len() {
        hamming_distance +=
            (bytes1.get(i).unwrap_or(&0) ^ bytes2.get(i).unwrap_or(&0)).count_ones();
    }

    return hamming_distance;
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
                    ascii::bytes_to_string(&decrypted_bytes),
                    english_score(&decrypted_bytes),
                )
            } else {
                (char, ascii::bytes_to_string(&decrypted_bytes), 0)
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
