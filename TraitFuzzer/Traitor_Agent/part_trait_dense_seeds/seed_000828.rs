#![feature(impl_trait_in_bindings)]

struct A<'a>(&'a ());

trait Trait<T> {
}

impl<T> Trait<T> for () {
}

trait ExtendedTrait<T>: Trait<T> {
    fn extended_method(&self) {}
}

impl<T, U: Trait<T>> ExtendedTrait<T> for U {}

fn main() {
    let x: impl ExtendedTrait<A> = ();
}