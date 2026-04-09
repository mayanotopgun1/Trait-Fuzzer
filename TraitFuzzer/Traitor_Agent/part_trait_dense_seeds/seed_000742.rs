#![feature(impl_trait_in_assoc_type)]
#![deny(dead_code)]

trait Fooable {
    fn foo(self);
}

impl Fooable for () {
    fn foo(self) { panic!(); }
}

fn main() {
    ().foo();
}