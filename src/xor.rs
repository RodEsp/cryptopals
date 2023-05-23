pub fn vecs(vec1: &Vec<u8>, vec2: &Vec<u8>) -> Vec<u8> {
    vec1.iter()
        .zip(vec2.iter())
        .map(|(x1, x2)| x1 ^ x2)
        .collect()
}

pub fn vec_against_char(vec: &Vec<u8>, char: char) -> Vec<u8> {
    let byte: u8 = char as u8;

    vec.into_iter().map(|hex_byte| hex_byte ^ byte).collect()
}
