// This gives us an array where each item is the ASCII decimal value of the corresponding char shown.
const BASE16_ALPHABET: [u8; 16] = *b"0123456789ABCDEF";

// The return value's bits will be Big Endian
pub fn encode(hex_string: &str) -> Vec<u8> {
    let bytes = (0..hex_string.len()) // Create an iterator from 0 to hex_string.len()
        .step_by(2) // In a hex string each byte is two decimals, so we'll step over the hex string by 2 chars each time
        .map(|i| {
            let hex_byte = &hex_string[i..=i + 1];
            u8::from_str_radix(hex_byte, 16) // u8::from_str_radix allows us to turn a string in any numerical base into a u8 in base10 (which is one byte (8 bits))
                .expect(&format!("Failed to convert 0x{} into a decimal.", hex_byte))
        })
        .collect::<Vec<u8>>();

    return bytes;
}

// Will return a string in the same order as the bits in the Vec<u8> that is passed in
// For example if you provide a single u8 with the following bits: 00010010 you will get back "12" since 0001 maps to "1" and 0010 maps to "2".
// Likewise, this vec:    vec![u8::from_str_radix("00010010", 2).unwrap(), u8::from_str_radix("00010010", 2).unwrap()];
//           will return: "1212"
pub fn decode(bytes: &Vec<u8>) -> String {
    let hex_string: String = bytes
        .into_iter()
        .flat_map(|byte| {
            // In a hex string each byte is two decimals, so we split the u8 byte into two 4-bit nibbles
            // We do this by using bit shifts to zero out the parts of the u8 we don't want for each nibble
            // And then store the resulting 4-bits in a new u8
            let nibbles: [u8; 2] = [byte >> 4, (byte << 4) >> 4];

            nibbles
                .into_iter()
                .map(|nibble| BASE16_ALPHABET[nibble as usize] as char) // BASE16_ALPHABET[*byte as usize] gives us the ASCII decimal value of a char so we need to cast it to char to get the hexadecimal string
        })
        .collect();

    return hex_string;
}
