#![feature(specialization)]

pub trait Foo {
    fn foo();
}

trait Bar: Foo {}

impl<T> Bar for T where T: Foo {}

impl Foo for i32 {}
impl Foo for i64 {}
impl<T> Foo for T {
    default fn foo() {}
}

fn main() {
    i32::foo();
    i64::foo();
    u8::foo();
}