pub trait Pad {
    fn pad(self: &mut Self, number_of_bytes: usize) -> &mut Self;
}

impl Pad for Vec<u8> {
    fn pad(self: &mut Self, number_of_bytes: usize) -> &mut Self {
        for _n in 0..number_of_bytes {
            self.push(0x04);
        }

        self
    }
}
