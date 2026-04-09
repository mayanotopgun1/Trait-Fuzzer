#![feature(const_trait_impl)]

const trait Barable {
    fn bar();
}

struct Foo;

impl const Barable for Foo {
    fn bar() {}
}

const _: () = Foo::bar();

fn main() {
    Foo::bar();
}