// This gives us an array where each item is the ASCII decimal value of the corresponding char shown.
const B64_ALPHABET: [u8; 64] = *b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

pub fn _encode(_base64_string: &str) -> Vec<u8> {
    todo!();
}

pub fn decode(bytes: &Vec<u8>) -> String {
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
