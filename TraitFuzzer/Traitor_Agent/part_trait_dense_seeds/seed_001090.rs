#![allow(non_camel_case_types)]

trait ClamTrait<T> {
    fn new(value: T) -> Self;
}

enum clam<T> {
    a(T),
}

impl<T> ClamTrait<T> for clam<T> {
    fn new(value: T) -> Self {
        clam::a(value)
    }
}

pub fn main() {
    let _c = clam::<i32>::new(3);
}