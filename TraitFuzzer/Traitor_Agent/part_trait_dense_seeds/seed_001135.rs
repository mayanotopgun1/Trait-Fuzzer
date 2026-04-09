#![feature(const_trait_impl)]

pub const trait Foo<Rhs: ?Sized = Self> {
    fn foo_method(&self) -> bool { true }
}

impl const Foo for () {}

trait FooExt: Foo {
    fn extended_foo(&self) -> bool { self.foo_method() }
}

impl<T: Foo> FooExt for T {}

fn main() {}