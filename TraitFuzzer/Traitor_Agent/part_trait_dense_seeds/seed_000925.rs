#![allow(dead_code)]

trait ArrayFactory<T> {
    fn create(t: T) -> impl Iterator<Item = T>;
}

impl<T> ArrayFactory<T> for () {
    fn create(t: T) -> impl Iterator<Item = T> {
        std::iter::once(t)
    }
}

trait U8ArrayFactory {
    fn create() -> impl Iterator<Item = u8>;
}

impl U8ArrayFactory for () {
    fn create() -> impl Iterator<Item = u8> {
        std::iter::once(99)
    }
}

fn main() {
    println!("{:?}", <() as ArrayFactory<i32>>::create(42).collect::<Vec<_>>());
    println!("{:?}", <() as U8ArrayFactory>::create().collect::<Vec<_>>());
}