const C: &'static [u8; 5] = b"hello";

trait Access {
    fn index(&self, i: usize) -> u8;
}

impl Access for &'static [u8; 5] {
    fn index(&self, i: usize) -> u8 {
        self[i]
    }
}

#[allow(unconditional_panic)]
fn mir() -> u8 {
    C.index(10)
}

fn main() {
    mir();
}