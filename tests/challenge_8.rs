use std::io::BufRead;

use cryptopals::hex;
use cryptopals::utilities::line_buffer;

#[test]
fn detect_aes_128_ecb_encryption() {
    let file = line_buffer("./data/challenge8.txt");

    let mut line_scores = Vec::<(usize, usize, String)>::new();

    for (index, hex_string) in file.lines().enumerate() {
        let line_number = index + 1;
        let encrypted_bytes = hex::string_to_bytes(
            &hex_string
                .as_ref()
                .expect("Could not read hex string from file."),
        );

        let mut score = 0;

        // Compare every encrypted chunk of 16 bytes from the hex string to every other 16 byte chunk in it.
        // Since ECB ciphers are deterministic - they produce the same ciphertext from the same plain text -
        //  and English text repeats words or sequences of words
        //  we should expect SOME of these chunks to match if the hex string was encrypted with ECB.
        for chunk in encrypted_bytes.chunks(16) {
            for next_chunk in encrypted_bytes.chunks(16) {
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
