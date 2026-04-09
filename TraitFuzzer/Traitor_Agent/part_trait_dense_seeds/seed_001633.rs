trait Accessor {
    fn get(&self) -> u8;
}

#[cfg(false)]
impl Accessor for u8 {
    fn get(&self) -> u8 {
        *self
    }
}

fn main() {}