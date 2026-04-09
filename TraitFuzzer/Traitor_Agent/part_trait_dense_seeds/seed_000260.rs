#![allow(warnings)]

trait Foo {
    fn borrowed<'a>(&'a self) -> &'a ();
}

trait FooExt: Foo {}

impl<T: Foo> FooExt for T {}

fn borrowed_receiver_related_lifetimes<'a, 'b>(x: &'a (dyn Foo + 'b)) -> &'a () {
    x.borrowed()
}

fn main() {}