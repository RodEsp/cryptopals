use cryptopals::base64;
use cryptopals::utilities::{
    find_single_char_repeating_xor_key_and_message, get_key_size_likelihoods, read_file,
};

#[test]
fn find_key_for_text_encrypted_with_repeating_key_xor_cipher() {
    // First we read the encrypted data from a file
    let base64_string = read_file("./data/challenge6.txt");
    // This is our encrypted data in raw bytes
    let encrypted_bytes = base64::string_to_bytes(&base64_string);

    let most_likely_key_size = get_key_size_likelihoods(&encrypted_bytes)[0].key_size;
    /* Now that we probably know the correct key size, we break up the encrypted data into byte chunks of KEYSIZE length.
    Each one of these chunks has been encrypted with repeating key XOR using the same key. */
    let key_size_chunks = encrypted_bytes.chunks(most_likely_key_size);

    /* We know that each one of these chunks was encrypted with a repeating key XOR,
       that means that each byte with the same index in each of these chunks was encrypted with the same single character
    So to find the repeating key that was used for the encrypted data we will group all bytes in each key sized chunk by their index
    Then we can solve each of these group as if it was a single char repeating XOR. */
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
