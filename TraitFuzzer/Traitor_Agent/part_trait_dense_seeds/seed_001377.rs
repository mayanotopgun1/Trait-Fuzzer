#![deny(unused)]

pub struct F;
pub struct B;

pub trait Createable {
    fn create() -> Self;
}

impl Createable for F {
    fn create() -> Self {
        F
    }
}

impl Createable for B {
    fn create() -> Self {
        B
    }
}

pub enum E {
    Foo(F),
    Bar(B),
}

fn main() {
    let _ = E::Foo(F::create());
}