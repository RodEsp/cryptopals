pub fn vecs(vec1: &Vec<u8>, vec2: &Vec<u8>) -> Vec<u8> {
    vec1.iter()
        .zip(vec2.iter())
        .map(|(x1, x2)| x1 ^ x2)
        .collect()
}

pub fn repeating_char(char: char, vec: &Vec<u8>) -> Vec<u8> {
    vec.into_iter().map(|byte| byte ^ (char as u8)).collect()
}

pub fn repeating_key(key: &Vec<u8>, vec: &Vec<u8>) -> Vec<u8> {
    let mut key_repeater = key.into_iter().cycle();

    vec.into_iter()
        .map(|hex_byte| {
            let encrypted_byte = hex_byte ^ key_repeater.next().unwrap();
            return encrypted_byte;
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::hex;

    use super::*;

    #[test]
    fn xor_bytes() {
        let vec1 = hex::string_to_bytes("01");
        let vec2 = hex::string_to_bytes("02");
        let xor_ed_vec = hex::string_to_bytes("03");
        assert_eq!(vecs(&vec1, &vec2), xor_ed_vec);
    }

    #[test]
    fn xor_multiple_bytes() {
        let vec1 = hex::string_to_bytes("01020102");
        let vec2 = hex::string_to_bytes("02010303");
        let xor_ed_vec = hex::string_to_bytes("03030201");
        assert_eq!(vecs(&vec1, &vec2), xor_ed_vec);
    }
}
