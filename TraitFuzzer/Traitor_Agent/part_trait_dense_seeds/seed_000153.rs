#![allow(warnings)]
#![feature(specialization)]

trait Bar<'a> {
    fn bar(&self, x: &str, y: &'a str);
}

default impl<'a, T> Bar<'a> for T {
    default fn bar(&self, _x: &str, _y: &'a str) {}
}

impl<'a> Bar<'a> for &'a str {
    fn bar(&self, _x: &str, _y: &'a str) {}
}

trait Foo<T> {}

fn main() {
}