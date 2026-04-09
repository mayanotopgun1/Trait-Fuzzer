#![feature(const_trait_impl)]

const trait Bar {
    fn bar();
}

impl const Bar for Foo {
    fn bar() {}
}

const _: () = Foo::bar();

struct Foo;

fn main() {
    let foo = Foo;
    Foo::bar();
}