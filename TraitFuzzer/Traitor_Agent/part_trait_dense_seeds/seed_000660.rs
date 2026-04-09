#![feature(generic_associated_types)]

pub struct X([u8]);

trait Length {
    type Out;
    fn length(&self) -> Self::Out;
}

impl Length for X {
    type Out = usize;
    fn length(&self) -> Self::Out {
        self.0.len()
    }
}

fn _f(x: &X) -> <X as Length>::Out {
    x.length()
}

fn main() {
    let b: &[u8] = &[11; 42];
    let v: &X = unsafe { std::mem::transmute(b) };
    assert_eq!(_f(v), 42);
}