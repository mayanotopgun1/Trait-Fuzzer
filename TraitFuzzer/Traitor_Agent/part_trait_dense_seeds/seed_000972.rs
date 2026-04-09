#![feature(specialization)]

trait Testable {
    fn test(&self, foo: bool) -> u8;
}

default impl<T> Testable for T {
    fn test(&self, foo: bool) -> u8 {
        match foo {
            true => 42,
            false => 0,
        }
    }
}

impl Testable for () {
    fn test(&self, foo: bool) -> u8 {
        match foo {
            true => *Box::new(9),
            false => 0,
        }
    }
}

fn main() {
    assert_eq!(9, ().test(true));
}