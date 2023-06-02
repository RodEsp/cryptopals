use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::{ascii, xor};

pub fn read_file(path: &str) -> String {
    let file = File::open(path).expect(&format!("Was not able to read {}", path));
    let reader = BufReader::new(file);
    reader
        .lines()
        // Remove newlines from file
        .map(|line| {
            let mut s = line.unwrap_or_default();
            let len = s.trim_end_matches(&['\r', '\n'][..]).len();
            s.truncate(len);
            return s;
        })
        .collect()
}

pub fn line_buffer(path: &str) -> impl std::io::BufRead {
    let file = File::open(path).expect(&format!("Was not able to read {}", path));
    BufReader::new(file)
}

pub fn hamming_distance(bytes1: &[u8], bytes2: &[u8]) -> u32 {
    if bytes1.len() != bytes2.len() {
        return u32::MAX;
    }

    let mut h_dist = 0;

    for i in 0..bytes1.len() {
        h_dist += (bytes1.get(i).expect("Can not compare the hamming distance")
            ^ bytes2.get(i).expect("Can not compare the hamming distance"))
        .count_ones();
    }

    return h_dist;
}

// Checks how many times the 13 most common english language characters appear in a string (represented by its individual bytes) and returns their count
pub fn english_score(vec: &Vec<u8>) -> usize {
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

pub fn find_single_char_repeating_xor_key_and_message(
    bytes: &Vec<u8>,
) -> (Option<char>, Option<String>, usize) {
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

pub trait Pad {
    fn pad(&mut self, number_of_bytes: usize) -> Self;
}

impl Pad for Vec<u8> {
    fn pad(self: &mut Self, number_of_bytes: usize) -> Self {
        for _n in 0..number_of_bytes {
            self.push(0x04);
        }

        self.to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hamming_distance_test() {
        assert_eq!(
            hamming_distance(
                &ascii::string_to_bytes("this is a test"),
                &ascii::string_to_bytes("wokka wokka!!!")
            ),
            37
        );
    }
}
