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

pub fn string_to_bytes(text: &str) -> Vec<u8> {
    text.as_bytes().to_vec()
}

// Takes a vec of bytes and turns it into an ASCII encoded string
pub fn bytes_to_string(bytes: &Vec<u8>) -> String {
    match String::from_utf8(bytes.to_vec()) {
        Ok(s) => Some(s).expect("Could not convert bytes into ASCII string"),
        Err(_e) => {
            // eprintln!("{:?}", e);
            "".to_string()
        }
    }
}
