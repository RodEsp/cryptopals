// This gives us an array where each item is the ASCII decimal value of the corresponding char shown.
const B64_ALPHABET: [u8; 64] = *b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

// The mapping in char_index represents how ASCII chars map to 6-bit bytes in the base64 encoding spec (see the table at https://en.wikipedia.org/wiki/Base64)
// All of the nums below are u8's (8-bits) whose first 2 bits can be ignored to give us the 6-bit "byte" that corresponds to the Base64 encoding
pub fn char_index(char: char) -> u8 {
    match char {
        'A' => 0,
        'B' => 1,
        'C' => 2,
        'D' => 3,
        'E' => 4,
        'F' => 5,
        'G' => 6,
        'H' => 7,
        'I' => 8,
        'J' => 9,
        'K' => 10,
        'L' => 11,
        'M' => 12,
        'N' => 13,
        'O' => 14,
        'P' => 15,
        'Q' => 16,
        'R' => 17,
        'S' => 18,
        'T' => 19,
        'U' => 20,
        'V' => 21,
        'W' => 22,
        'X' => 23,
        'Y' => 24,
        'Z' => 25,
        'a' => 26,
        'b' => 27,
        'c' => 28,
        'd' => 29,
        'e' => 30,
        'f' => 31,
        'g' => 32,
        'h' => 33,
        'i' => 34,
        'j' => 35,
        'k' => 36,
        'l' => 37,
        'm' => 38,
        'n' => 39,
        'o' => 40,
        'p' => 41,
        'q' => 42,
        'r' => 43,
        's' => 44,
        't' => 45,
        'u' => 46,
        'v' => 47,
        'w' => 48,
        'x' => 49,
        'y' => 50,
        'z' => 51,
        '0' => 52,
        '1' => 53,
        '2' => 54,
        '3' => 55,
        '4' => 56,
        '5' => 57,
        '6' => 58,
        '7' => 59,
        '8' => 60,
        '9' => 61,
        '+' => 62,
        '/' => 63,
        _ => panic!("Invalid character passed to char_index"),
    }
}
pub fn string_to_bytes(base64_string: &str) -> Vec<u8> {
    // Base64 strings are encoded in binary as 6-bit chunks. So for each 8-bit byte we only care about its first 6 bits.
    let bytes = base64_string
        .chars()
        .collect::<Vec<char>>()
        .chunks(4) // Split the chars of the base64 string into chunks of 4. This way we can assign 6-bits of each to a 24-bit chunks (6 * 4 = 24) and later on, split those 24-bits into even 8-bit bytes (24 / 8 = 3)
        .map(|chunk| {
            // This map turns our chunks of 4 bytes into the 8-bit bytes that correspond to the binary encoding of a base64 string by using the char_index mapping above
            chunk
                .into_iter()
                .filter(|char| *char != &'=') // '=' is a padding char in a base64 string so we will filter it out here since it doesn't have any actual bit encoding
                .map(|char| char_index(*char))
                .collect()
        })
        // Now that we have a set of bytes that correspond to the base64 encoding we will us bit shifting to "throw away" the unnecessary bits
        // this will allow us to pull out the relevant 24-bits in four 8-bit bytes and stuff them into a new set of three 8-bit bytes.
        .flat_map(|bytes: Vec<u8>| match bytes.len() {
            // We match on the length of the byte vector in case there were padding chars, '=', in our base64 string that left us without a clean multiple of 4 for the last chunk in the map statement above
            2 => vec![bytes[0] << 2 | bytes[1] >> 4, bytes[1] << 4],
            3 => vec![
                bytes[0] << 2 | bytes[1] >> 4,
                bytes[1] << 4 | bytes[2] >> 2,
                bytes[2] << 6,
            ],
            4 => vec![
                bytes[0] << 2 | bytes[1] >> 4,
                bytes[1] << 4 | bytes[2] >> 2,
                bytes[2] << 6 | bytes[3],
            ],
            _ => unreachable!(),
        })
        .collect();

    return bytes;
}

pub fn bytes_to_string(bytes: &Vec<u8>) -> String {
    let b64_string: String = bytes
        .chunks(3)
        .flat_map(|chunk| {
            // chunk is a slice of three values from bytes (u8's)
            // we will take these three u8's and make a 24-bit chunk out of them
            // so that we can then split those 24 bits into 6-bit chunks
            // to transform them into base64

            // println!(
            //     "chunk in binary:\n {:0>8b}, {:0>8b}, {:0>8b}\n",
            //     chunk[0], chunk[1], chunk[2]
            // );

            // Using ::from_be_bytes we are stuffing the three u8's into a u32 and padding the end with 0s
            // We are using _be_ (big endian) so that the bits are stored from left to right in natural human reading order (at least for english)
            let bits = u32::from_be_bytes([chunk[0], chunk[1], chunk[2], 0]);

            // println!("chunk stuffed into a u32 in binary:\n {:0>32b}\n", bits);

            // Use bitwise left and right shift operators to split the first 24-bits of the u32 above into 6-bit chunks
            // these are stored in u8s so the left side of the 6-bits is padded with two zeros
            let base64_bytes: [u8; 4] = [
                (bits >> 26) as u8,
                ((bits << 6) >> 26) as u8,
                ((bits << 12) >> 26) as u8,
                ((bits << 18) >> 26) as u8,
            ];

            // println!(
            //     "bits split up into 6-bit chunks:\n {:0>6b}, {:0>6b}, {:0>6b}, {:0>6b}\n",
            //     base64_bytes[0], base64_bytes[1], base64_bytes[2], base64_bytes[3]
            // );

            // Get the base64 encoded string by using the 6-bit bytes as indexes for the B64_ALPHABET array
            base64_bytes
                .into_iter()
                .map(|byte| B64_ALPHABET[byte as usize] as char) // B64_ALPHABET[*byte as usize] gives us the ASCII decimal value of a char so we need to cast it to char to display the base64 encoded string
        })
        .collect();

    return b64_string;
}
