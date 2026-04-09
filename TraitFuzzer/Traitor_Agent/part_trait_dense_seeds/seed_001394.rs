struct S (
    #[rustfmt::skip]
    #[rustfmt::skip]
    u8
);

trait Value {
    fn value(&self) -> u8;
}

impl Value for S {
    fn value(&self) -> u8 {
        self.0
    }
}

fn main() {}