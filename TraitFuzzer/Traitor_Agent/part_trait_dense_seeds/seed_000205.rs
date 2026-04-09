trait Multiply<T> {
    fn multiply(self, other: T) -> u16;
}

impl Multiply<u8> for u8 {
    fn multiply(self, other: u8) -> u16 {
        (self as u16) * (other as u16)
    }
}

fn main() {
    let x = 200u8.multiply(4);
}