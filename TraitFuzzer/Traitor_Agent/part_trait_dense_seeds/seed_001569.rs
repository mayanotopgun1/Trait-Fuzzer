#![feature(specialization)]
#![allow(dead_code)]
#![allow(unused_variables)]

trait Droppable {}
default impl<T> Droppable for T {}

enum Foo {}

impl Drop for Foo {
    fn drop(&mut self) {}
}

trait Foos {
    fn foo(self);
}
impl Foos for Foo {
    default fn foo(self) {}
}

fn main() {}