#![allow(dead_code)]
#![allow(unused_variables)]

trait Droppable {}
impl Droppable for Foo {}

enum Foo {}

impl Drop for Foo {
    fn drop(&mut self) {}
}

trait Foos {
    fn foo(self);
}
impl Foos for Foo {
    fn foo(self) {}
}

fn main() {}