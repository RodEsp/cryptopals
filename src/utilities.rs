use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use openssl::rand::rand_bytes;

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
    let mut h_dist = 0;

    for i in 0..bytes1.len() {
        h_dist += (bytes1.get(i).expect("Can not compare the hamming distance")
            ^ bytes2.get(i).expect("Can not compare the hamming distance"))
        .count_ones();
    }

    return h_dist;
}

pub struct KeySizeWithNormalizedHammingDistance {
    pub key_size: usize,
    normalized_hamming_distance: f32,
}

pub fn get_key_size_likelihoods(
    encrypted_bytes: &Vec<u8>,
) -> Vec<KeySizeWithNormalizedHammingDistance> {
    /* Here we find the most likely key size that was used to encrypt the the data
     * To do this, for each possible key size from 0 to KEYSIZE, we take two consecutive chunks of the same number of bytes from the encrypted bytes
     *     chunk_1 = encrypted_bytes[0..KEYSIZE]
     *     chunk_2 = encrypted_bytes[KEYSIZE..KEYSIZE * 2]
     * then we find the hamming distance between them and normalize it by dividing by it with the key size
     *     normalized_hamming_distance = hamming_distance(chunk_1, chunk_2)
     * The key size that produces the smallest normalized hamming distance is probably the key.
     * For an explanation of why this works see https://crypto.stackexchange.com/questions/8115/repeating-key-xor-and-hamming-distance/8118#8118
     */
    let min_key_size = 2;
    let max_key_size = 40;

    let mut key_size_likelihoods = Vec::<KeySizeWithNormalizedHammingDistance>::new(); // Stores each key size with a numerial score that corresponds to the normalized hamming distance it produced

    for key_size in min_key_size..=max_key_size {
        key_size_likelihoods.push(KeySizeWithNormalizedHammingDistance {
            key_size,
            normalized_hamming_distance: hamming_distance(
                // Here we multiply the chunk sizes by 4 in order to make the chunks corresponding to each key size bigger
                // This gives us an advantage as explained in the link above.
                &encrypted_bytes[0..(key_size * 4)],
                &encrypted_bytes[(key_size * 4)..(key_size * 2 * 4)],
            ) as f32
                / key_size as f32,
        });
    }

    // Sort the key sizes by most likely based on their normalized hamming distances
    key_size_likelihoods.sort_by(|a, b| {
        a.normalized_hamming_distance
            .total_cmp(&b.normalized_hamming_distance)
    });

    return key_size_likelihoods;
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
            let decrypted_bytes = xor::repeating_char(char, bytes);

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
            most_likely_key_and_message = (Some(item.0), Some(item.1), item.2);
        }
    });

    return most_likely_key_and_message;
}

pub fn random_bytes(num_of_bytes: usize) -> Vec<u8> {
    let mut bytes: Vec<u8> = vec![0; num_of_bytes];

    let mut bytes_slice = bytes.as_mut_slice();
    rand_bytes(&mut bytes_slice).expect("Could not generate random bytes");

    bytes
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

    #[test]
    fn generate_random_key() {
        let key = random_bytes(16);

        assert_eq!(key.len(), 16);
        assert!(key.into_iter().any(|byte| byte != 0 as u8))
    }
}
