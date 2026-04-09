const C: &'static [u8; 5] = b"hello";

trait Access {
    fn index(&self, i: usize) -> impl core::fmt::Debug;
}

impl Access for &'static [u8; 5] {
    fn index(&self, i: usize) -> impl core::fmt::Debug {
        self[i]
    }
}

#[allow(unconditional_panic)]
fn mir() -> impl core::fmt::Debug {
    C.index(10)
}

fn main() {
    mir();
}