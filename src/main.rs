mod ascii;
mod base64;
mod hex;
mod xor;

fn main() {
    println!("Challenge 1: {}", base64::encode(hex::decode("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d")));

    println!(
        "Challenge 2: {}",
        hex::encode(
            xor::vecs(
                hex::decode("686974207468652062756c6c277320657965"),
                hex::decode("1c0111001f010100061a024b53535009181c")
            )
            .into()
        )
    );

    // Create a vec of tuples that contain the character, message, and a score
    // The score notes the likelyhood that the message is the secret message
    // Which also means taht the character is our secret key
    let scores = ascii::alphabet()
        .into_iter()
        .map(|char| {
            // XOR the given hex_string against each ASCII character
            let bytes = xor::vec_against_char(
                hex::decode("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736"),
                char,
            );

            // Take the resulting bytes and figure out which one is the most likely to be a secret message, written in English
            if bytes
                .clone()
                .into_iter()
                // Filter for strings that only contain chars that exist in the english language
                .all(|byte| matches!(byte as char, ' '..='}'))
            {
                // Give a score to each secret key and message that states how likely it is to be the secret info we're after
                (char, ascii::encode(&bytes), english_score(&bytes))
            } else {
                (char, ascii::encode(&bytes), 0)
            }
        })
        .collect::<Vec<_>>();

    let mut most_likely_key_and_message: (Option<char>, Option<String>, usize) = (None, None, 0);

    scores.into_iter().for_each(|item| {
        if item.2 > most_likely_key_and_message.2 {
            most_likely_key_and_message = ((item.0).into(), (item.1).into(), item.2);
        }
    });

    print!(
        "Challenge 3:\n Key = '{}', Message = \"{}\"",
        most_likely_key_and_message.0.unwrap(),
        most_likely_key_and_message.1.unwrap()
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
