#![feature(const_trait_impl, specialization)]

pub const trait Foo<Rhs: ?Sized = Self> {
    fn foo_method(&self) -> bool { true }
}

impl const Foo for () {}

trait FooExt: Foo {
    fn extended_foo(&self) -> bool;
}

default impl<T: Foo> FooExt for T {
    default fn extended_foo(&self) -> bool { self.foo_method() }
}

fn main() {}