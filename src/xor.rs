use crate::hex;

pub fn fixed(buffer: Vec<u8>) -> Vec<u8> {
    let fixed_buffer: Vec<u8> = hex::decode("686974207468652062756c6c277320657965");

    fixed_buffer
        .iter()
        .zip(buffer.iter())
        .map(|(x1, x2)| x1 ^ x2)
        .collect()
}
