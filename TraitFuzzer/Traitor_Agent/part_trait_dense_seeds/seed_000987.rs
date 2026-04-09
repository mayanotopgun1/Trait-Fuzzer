#![feature(impl_trait_in_assoc_type)]

struct Foo<T, U>(T, U);

trait FromArray {
    type Array;
}

impl<T> FromArray for Foo<T, [u8; 2]> {
    type Array = [u8; 2];
}

impl<T> From<[u8; 2]> for Foo<T, [u8; 2]> {
    fn from(value: [u8; 2]) -> Foo<T, [u8; 2]> {
        todo!();
    }
}

fn break_me<T>()
where
    Foo<T, [u8; 2]>: From<[u8; 2]>,
{}

fn main() {}