// Returns an array containing all the standard ASCII chars in ascending order
pub fn alphabet() -> [char; 128] {
    let mut ascii_alphabet = ['0'; 128];
    ascii_alphabet.copy_from_slice(
        (0..128)
            .map(|digit| char::from(digit))
            .collect::<Vec<char>>()
            .as_slice(),
    );

    return ascii_alphabet;
}

// Takes a vec of bytes and turns it into an ASCII encoded string
pub fn encode(bytes: &Vec<u8>) -> String {
    match std::str::from_utf8(&bytes) {
        Ok(c) => c.to_string(),
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    }
}
