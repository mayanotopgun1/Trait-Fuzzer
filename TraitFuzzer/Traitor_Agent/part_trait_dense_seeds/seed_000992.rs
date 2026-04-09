#![allow(dead_code)]
#![allow(unconditional_recursion)]

trait Foo {
}

trait FooExt: Foo {
    fn foo_ext(&self) {}
}

impl<T: Foo> FooExt for T {}

fn b(_x: Box<dyn FooExt+Send>) {
}

fn c(x: Box<dyn FooExt+Sync+Send>) {
    e(x);
}

fn d(x: Box<dyn FooExt+Send>) {
    e(x);
}

fn e(x: Box<dyn FooExt>) {
    x.foo_ext();
}

pub fn main() { }