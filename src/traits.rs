use crate::utilities::random_bytes;

const PKCS7_PADDING_BYTES: [u8; 16] = [
    0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10,
];

pub trait Pkcs7Padding {
    fn add_pkcs7_padding(self: &mut Self) -> &mut Self;
    fn remove_pkcs7_padding(self: &mut Self) -> &mut Self;
}

impl Pkcs7Padding for Vec<u8> {
    fn add_pkcs7_padding(self: &mut Self) -> &mut Self {
        let num_of_bytes_needed = 16 - (self.len() % 16);

        self.extend(vec![
            PKCS7_PADDING_BYTES[num_of_bytes_needed - 1];
            num_of_bytes_needed
        ]);

        self
    }

    fn remove_pkcs7_padding(self: &mut Self) -> &mut Self {
        let last_byte = self.last().unwrap();

        if PKCS7_PADDING_BYTES.contains(last_byte) {
            for _i in 0..*last_byte {
                self.pop();
            }
        }

        self
    }
}

pub trait PadRand {
    fn left_pad_rand(self: &mut Self, num_of_bytes: usize) -> &mut Self;
    fn right_pad_rand(self: &mut Self, num_of_bytes: usize) -> &mut Self;
}

impl PadRand for Vec<u8> {
    fn left_pad_rand(self: &mut Self, num_of_bytes: usize) -> &mut Self {
        self.splice(0..0, random_bytes(num_of_bytes));
        self
    }

    fn right_pad_rand(self: &mut Self, num_of_bytes: usize) -> &mut Self {
        self.extend(random_bytes(num_of_bytes));
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn append_and_prepend_random_bytes() {
        let mut bytes = vec![0, 0, 0];

        bytes.left_pad_rand(5);
        bytes.right_pad_rand(7);

        assert_eq!(bytes.len(), 15);
        assert!(bytes.into_iter().any(|byte| byte != 0))
    }
}
