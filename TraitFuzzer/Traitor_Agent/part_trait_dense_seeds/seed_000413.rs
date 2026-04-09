#![feature(const_trait_impl)]

const trait Foo {
    fn bar();
}

struct S;

impl Foo for S {
    fn bar() {}
}

trait FooExt: Foo {}

impl<T> FooExt for T where T: Foo {}

fn baz<T: FooExt>() {
    T::bar();
}

const fn qux<T: FooExt + [const] Foo>() {
    T::bar();
}

fn main() {}