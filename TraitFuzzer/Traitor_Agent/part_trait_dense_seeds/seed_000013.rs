#![feature(negative_impls)]

trait Foo {
    fn foo() {}
}

trait FooExt: Foo {
    fn foo_ext(&self) where Self: Sized {
        Self::foo(); // Use associated function syntax instead of method syntax
    }
}

impl<T> FooExt for T where T: Foo {}

impl !Foo for () {}

fn main() {}