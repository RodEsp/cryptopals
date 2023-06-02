pub mod ascii;
pub mod base64;
pub mod hex;
pub mod utilities;
pub mod xor;

#[cfg(test)]
mod tests {
    use std::{io::BufRead, vec};

    use openssl::symm::{decrypt, Cipher};

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
        let (key, message, ..) =
            find_single_char_repeating_xor_key_and_message(&hex::string_to_bytes(
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
        // First we read the encrypted data from a file
        let base64_string = read_file("./data/challenge6.txt");

        // This is our encrypted data in raw bytes
        let encrypted_bytes = base64::string_to_bytes(&base64_string);

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

        struct KeySizeWithNormalizedHammingDistance {
            key_size: usize,
            normalized_hamming_distance: f32,
        }
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

        // Now that we probably know the correct key size, we break up the encrypted data into byte blocks of KEYSIZE length.
        // Each one of these blocks has been encrypted with repeating key XOR using the same key.
        let most_likely_key_size = key_size_likelihoods[0].key_size;
        let key_size_chunks = encrypted_bytes.chunks(most_likely_key_size);

        /* We know that each one of these blocks was encrypted with a repeating key XOR,
         *  that means that each byte with the same index in each of these blocks was encrypted with the same single character
         * So to find the repeating key that was used for the encrypted data we will group all bytes in each key sized chunk by their index
         * Then we can solve each of these group as if it was a single char repeating XOR.
         */

        // Group the bytes in each key sized byte chunks by their index
        let bytes_grouped_by_chunk_index =
            key_size_chunks.fold(Vec::<Vec<u8>>::new(), |mut acc, chunk| {
                for (i, byte) in chunk.iter().enumerate() {
                    if i >= acc.len() {
                        acc.push(Vec::<u8>::new())
                    }
                    acc[i % most_likely_key_size].push(*byte);
                }
                return acc;
            });

        let mut key_chars: Vec<char> = vec![];

        // Guess the character that was used to encrypt each group of bytes
        bytes_grouped_by_chunk_index.into_iter().for_each(|bytes| {
            key_chars.push(
                find_single_char_repeating_xor_key_and_message(&bytes)
                    .0
                    .unwrap_or_default(),
            );
        });

        // Find the key that was used to encrypt the data by concatenating the single-char we guessed for each group of bytes.
        let key = key_chars.into_iter().collect::<String>();

        assert_eq!(key, "Terminator X: Bring the noise");

        // This prints the decoded text using the key we found
        // println!(
        //     "{}",
        //     ascii::bytes_to_string(&xor::repeating_key(
        //         &ascii::string_to_bytes(&key),
        //         &encrypted_bytes
        //     ))
        //     .unwrap()
        // );
    }

    // Challenge 7
    #[test]
    fn challenge_7_test() {
        // First we read the encrypted data from a file
        let base64_string = read_file("./data/challenge7.txt");

        // This is our encrypted data in raw bytes
        let encrypted_bytes = base64::string_to_bytes(&base64_string);
        let decrypted_bytes = decrypt(
            Cipher::aes_128_ecb(),
            b"YELLOW SUBMARINE",
            None,
            encrypted_bytes.as_slice(),
        )
        .unwrap();

        assert_eq!(decrypted_bytes, *b"I'm back and I'm ringin' the bell \nA rockin' on the mike while the fly girls yell \nIn ecstasy in the back of me \nWell that's my DJ Deshay cuttin' all them Z's \nHittin' hard and the girlies goin' crazy \nVanilla's on the mike, man I'm not lazy. \n\nI'm lettin' my drug kick in \nIt controls my mouth and I begin \nTo just let it flow, let my concepts go \nMy posse's to the side yellin', Go Vanilla Go! \n\nSmooth 'cause that's the way I will be \nAnd if you don't give a damn, then \nWhy you starin' at me \nSo get off 'cause I control the stage \nThere's no dissin' allowed \nI'm in my own phase \nThe girlies sa y they love me and that is ok \nAnd I can dance better than any kid n' play \n\nStage 2 -- Yea the one ya' wanna listen to \nIt's off my head so let the beat play through \nSo I can funk it up and make it sound good \n1-2-3 Yo -- Knock on some wood \nFor good luck, I like my rhymes atrocious \nSupercalafragilisticexpialidocious \nI'm an effect and that you can bet \nI can take a fly girl and make her wet. \n\nI'm like Samson -- Samson to Delilah \nThere's no denyin', You can try to hang \nBut you'll keep tryin' to get my style \nOver and over, practice makes perfect \nBut not if you're a loafer. \n\nYou'll get nowhere, no place, no time, no girls \nSoon -- Oh my God, homebody, you probably eat \nSpaghetti with a spoon! Come on and say it! \n\nVIP. Vanilla Ice yep, yep, I'm comin' hard like a rhino \nIntoxicating so you stagger like a wino \nSo punks stop trying and girl stop cryin' \nVanilla Ice is sellin' and you people are buyin' \n'Cause why the freaks are jockin' like Crazy Glue \nMovin' and groovin' trying to sing along \nAll through the ghetto groovin' this here song \nNow you're amazed by the VIP posse. \n\nSteppin' so hard like a German Nazi \nStartled by the bases hittin' ground \nThere's no trippin' on mine, I'm just gettin' down \nSparkamatic, I'm hangin' tight like a fanatic \nYou trapped me once and I thought that \nYou might have it \nSo step down and lend me your ear \n'89 in my time! You, '90 is my year. \n\nYou're weakenin' fast, YO! and I can tell it \nYour body's gettin' hot, so, so I can smell it \nSo don't be mad and don't be sad \n'Cause the lyrics belong to ICE, You can call me Dad \nYou're pitchin' a fit, so step back and endure \nLet the witch doctor, Ice, do the dance to cure \nSo come up close and don't be square \nYou wanna battle me -- Anytime, anywhere \n\nYou thought that I was weak, Boy, you're dead wrong \nSo come on, everybody and sing this song \n\nSay -- Play that funky music Say, go white boy, go white boy go \nplay that funky music Go white boy, go white boy, go \nLay down and boogie and play that funky music till you die. \n\nPlay that funky music Come on, Come on, let me hear \nPlay that funky music white boy you say it, say it \nPlay that funky music A little louder now \nPlay that funky music, white boy Come on, Come on, Come on \nPlay that funky music \n");
    }

    // Challenge 8
    #[test]
    fn challenge_8_test() {
        let file = line_buffer("./data/challenge8.txt");

        let mut line_scores = Vec::<(usize, usize, String)>::new();

        for (index, hex_string) in file.lines().enumerate() {
            let line_number = index + 1;
            let bytes = hex::string_to_bytes(
                &hex_string
                    .as_ref()
                    .expect("Could not read hex string from file."),
            );

            let mut score = 0;
            for chunk in bytes.chunks(16) {
                for next_chunk in bytes.chunks(16) {
                    if chunk == next_chunk {
                        score += 1;
                    }
                }
            }

            let line_score = (score, line_number, hex_string.unwrap());
            line_scores.push(line_score);
        }

        let highest_scoring_line = line_scores
            .into_iter()
            .max()
            .expect("Could not get the max score from lines");

        assert_eq!(highest_scoring_line.1, 133);
        assert_eq!(highest_scoring_line.2, "d880619740a8a19b7840a8a31c810a3d08649af70dc06f4fd5d2d69c744cd283e2dd052f6b641dbf9d11b0348542bb5708649af70dc06f4fd5d2d69c744cd2839475c9dfdbc1d46597949d9c7e82bf5a08649af70dc06f4fd5d2d69c744cd28397a93eab8d6aecd566489154789a6b0308649af70dc06f4fd5d2d69c744cd283d403180c98c8f6db1f2a3f9c4040deb0ab51b29933f2c123c58386b06fba186a");
    }

    // Challenge 9
    #[test]
    fn challenge_9_test() {
        let mut bytes = ascii::string_to_bytes("YELLOW SUBMARINE");

        assert_eq!(20, bytes.pad(4).len());
        assert_eq!(0x04, *bytes.last().unwrap());
    }
}
