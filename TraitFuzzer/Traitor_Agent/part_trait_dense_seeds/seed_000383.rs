#![allow(dead_code)]

trait FooTrait {
    fn new() -> Self;
    fn bar();
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