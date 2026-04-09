#![feature(rustc_attrs)]

#![deny(dead_code)]

trait Fooable {
    fn foo(self);
}

impl Fooable for () {
    fn foo(self) { panic!(); }
}

#[rustc_main]
fn main() {
    ().foo();
}