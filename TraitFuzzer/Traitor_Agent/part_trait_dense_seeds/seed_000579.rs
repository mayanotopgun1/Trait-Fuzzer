#![feature(specialization)]

use std::cell::RefCell;

trait Foo {
    fn foo(&self) {}
}

default impl<T> Foo for T where T: Clone {}

struct Bar;

impl Foo for Bar {
    fn foo(&self) {}
}

fn main() {
    let b = RefCell::new(Bar);
    b.borrow().foo();
}