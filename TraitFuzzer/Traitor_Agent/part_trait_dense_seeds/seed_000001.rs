#![feature(inherent_associated_types)]
#![allow(incomplete_features, dead_code, unused_variables)]

struct Foo;

trait BarFactory<const X: usize> {
    type Out;
    fn new() -> Self::Out;
}

impl<const X: usize> BarFactory<X> for Bar<X> {
    type Out = Self;
    fn new() -> Self::Out {
        Self([(); X])
    }
}

struct Bar<const X: usize>([(); X]);

impl Foo {
    type Bar<const X: usize> = Bar<X>;
}

fn main() {
    let a = <Bar<10usize> as BarFactory<10usize>>::new();
}