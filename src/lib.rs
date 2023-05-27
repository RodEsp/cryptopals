pub mod ascii;
pub mod base64;
pub mod hex;
pub mod utilities;
pub mod xor;

#[cfg(test)]
mod tests {
    use std::{
        fs::File,
        io::{BufRead, BufReader},
    };

    use crate::hex;
    use crate::utilities::*;
    use crate::xor;
    use crate::{ascii, base64};

    // Challenge 1
    #[test]
    fn challenge_1_test() {
        assert_eq!(base64::bytes_to_string(&hex::string_to_bytes("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d")), "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
    }

    // Challenge 2
    #[test]
    fn challenge_2_test() {
        assert_eq!(
            hex::bytes_to_string(&xor::vecs(
                &hex::string_to_bytes("686974207468652062756c6c277320657965"),
                &hex::string_to_bytes("1c0111001f010100061a024b53535009181c")
            )),
            "746865206b696420646f6e277420706c6179".to_uppercase()
        );
    }

    // Challenge 3
    #[test]
    fn challenge_3_test() {
        let (key, message, ..) = find_encryption_key_and_message(&hex::string_to_bytes(
            "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736",
        ));

        assert_eq!(key.unwrap_or_default(), 'X');
        assert_eq!(
            message.unwrap_or_default(),
            "Cooking MC's like a pound of bacon"
        );
    }

    // Challenge 4
    #[test]
    fn challenge_4_test() {
        let file = File::open("./data/challenge4.txt")
            .expect("Was not able to read ./data/challenge4.txt");
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

        assert_eq!(highest_scoring_key.unwrap_or_default(), '5');
        assert_eq!(
            highest_scoring_message.unwrap_or_default(),
            "Now that the party is jumping\n"
        );
    }

    // Challenge 5
    #[test]
    fn challenge_5_test() {
        assert_eq!(
            hex::bytes_to_string(&xor::repeating_key(
                &ascii::string_to_bytes("ICE"),
                &ascii::string_to_bytes(
                    "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal"
                )
            )), "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f".to_uppercase()
        );
    }

    // Challenge 6
    #[test]
    fn challenge_6_test() {
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
        
        let key_size = 2;
        let max_key_size = 40;

        // TODO: Find the most likely key_size being used to encrypt the ciphertext
        // For each KEYSIZE, take the first KEYSIZE worth of bytes, and the second KEYSIZE worth of bytes, 
        // and find the hamming distance between them. Normalize this result by dividing by KEYSIZE.
        // The KEYSIZE with the smallest normalized hamming distance is probably the key.

        // TODO: Now that you probably know the KEYSIZE: break the ciphertext into blocks of KEYSIZE length.

        // TODO: Transpose the blocks: make a block that is the first byte of every block,
        // and a block that is the second byte of every block, and so on.

        // TODO: Solve each block as if it was single-character XOR. You can use the utilities::find_encryption_key_and_message function for this.

        // TODO: Find the key used to encrypt the ciphertext by concatenating the key for each block.
        // For each block, the single-byte XOR key that produces the best looking histogram is the repeating-key XOR key byte for that block. Put them together and you have the key.

        assert_eq!("Some key", "Some message");
    }
}
