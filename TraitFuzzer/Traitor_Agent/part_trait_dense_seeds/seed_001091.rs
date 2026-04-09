#![allow(non_camel_case_types)]

trait ClamTrait<T> {
    fn new(value: T) -> Self;
}

#[derive(Debug)]
enum clam<T: std::fmt::Debug> {
    a(T),
}

impl<T: std::fmt::Debug> ClamTrait<T> for clam<T> {
    fn new(value: T) -> clam<T> {
        clam::a(value)
    }
}

pub fn main() {
    let _c = <clam<i32>>::new(3);
}