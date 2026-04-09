struct A<'a>(&'a ());

trait ArrayInitializer<const N: usize> {
    fn initialize_array(&self) -> [u8; N];
}

impl<'a> ArrayInitializer<68> for A<'a> {
    fn initialize_array(&self) -> [u8; 68] {
        let mut b = [0; 68];
        b
    }
}

fn main() {}