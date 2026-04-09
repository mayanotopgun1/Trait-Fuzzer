#![allow(unused_variables)]

trait Fooable {
    fn foo(self);
}

impl Fooable for (i8, i8) {
    fn foo(self) {}
}

fn main() {
    (0, 1).foo();
}