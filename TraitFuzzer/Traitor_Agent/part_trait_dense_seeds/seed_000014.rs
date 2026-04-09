#![feature(negative_impls, return_position_impl_trait_in_trait)]

trait Foo {
    fn foo() {}
}

trait FooExt: Foo {
    fn foo_ext(&self) -> impl Fn() where Self: Sized {
        move || Self::foo()
    }
}

impl<T> FooExt for T where T: Foo {}

impl !Foo for () {}

fn main() {}