#![feature(generic_associated_types)]

pub trait Foo<T> {
    type Out;
    fn foo(self) -> Self::Out;
}

trait Bar<T>: Foo<T> {
    fn bar(&self) -> Option<Self::Out> {
        None
    }
}

impl<S, T> Bar<T> for S where S: Foo<T> {}

impl<'a, T> Foo<T> for &'a str where &'a str: Into<T> {
    type Out = T;
    fn foo(self) -> Self::Out {
        panic!();
    }
}

fn main() {}