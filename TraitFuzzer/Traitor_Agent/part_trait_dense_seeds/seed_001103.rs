#![warn(dead_code)]

trait Callable {
    fn call(&self);
}

struct Bar;

impl Callable for Bar {
    fn call(&self) {}
}

fn foo<C: Callable>(c: C) {
    c.call()
}

fn main() {
    let bar = Bar;
    foo(bar);
}