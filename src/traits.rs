const PKCS7_PADDING_BYTES: [u8; 16] = [
    0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10,
];

pub trait Pad {
    fn pad_pkcs7(self: &mut Self) -> &mut Self;
}

impl Pad for Vec<u8> {
    fn pad_pkcs7(self: &mut Self) -> &mut Self {
        let num_of_bytes_needed = 16 - (self.len() % 16);

        for _i in 0..num_of_bytes_needed {
            self.push(PKCS7_PADDING_BYTES[num_of_bytes_needed - 1]);
        }

        self
    }
}
