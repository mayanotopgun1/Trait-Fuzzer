#![feature(specialization)]
#![allow(dead_code)]

trait FooTrait {
    fn new() -> Self;
    fn bar();
}

default impl<T> FooTrait for T {
    fn new() -> Self {
        unimplemented!()
    }

    fn bar() {
        unimplemented!()
    }
}

impl FooTrait for Foo {
    fn new() -> Self {
        Foo
    }

    fn bar() {
        Self::new();
    }
}

struct Foo;

fn main() {}