pub fn vecs(vec1: &Vec<u8>, vec2: &Vec<u8>) -> Vec<u8> {
    vec1.iter()
        .zip(vec2.iter())
        .map(|(x1, x2)| x1 ^ x2)
        .collect()
}

pub fn u8_against_char(byte: u8, char: char) -> u8 {
    byte ^ (char as u8)
}

pub fn vec_against_char(vec: &Vec<u8>, char: char) -> Vec<u8> {
    let byte: u8 = char as u8;

    vec.into_iter().map(|hex_byte| hex_byte ^ byte).collect()
}

pub fn repeating_key(key: &Vec<u8>, vec: &Vec<u8>) -> Vec<u8> {
    let key_length = key.len() - 1;
    let mut key_index = 0;

    vec.into_iter()
        .map(|hex_byte| {
            let encrypted_byte = hex_byte ^ key[key_index];

            if key_index == key_length {
                key_index = 0;
            } else {
                key_index += 1;
            }

            return encrypted_byte;
        })
        .collect()
}
