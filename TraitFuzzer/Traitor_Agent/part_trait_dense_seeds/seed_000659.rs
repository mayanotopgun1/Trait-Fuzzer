use std::mem;

pub struct X([u8]);

trait Length {
    fn length(&self) -> usize;
}

impl Length for X {
    fn length(&self) -> usize {
        self.0.len()
    }
}

fn _f(x: &X) -> usize {
    x.length()
}

fn main() {
    let b: &[u8] = &[11; 42];
    let v: &X = unsafe { mem::transmute(b) };
    assert_eq!(_f(v), 42);
}