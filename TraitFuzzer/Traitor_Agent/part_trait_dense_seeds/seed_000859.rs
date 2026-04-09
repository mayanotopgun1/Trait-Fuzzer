#![feature(const_trait_impl)]

const trait Barable {
    fn bar(&self);
}

struct Foo;

impl const Barable for Foo {
    fn bar(&self) {}
}

fn run(x: &dyn Barable) {
    x.bar();
}

fn main() {
    let foo = Foo;
    run(&foo);
}