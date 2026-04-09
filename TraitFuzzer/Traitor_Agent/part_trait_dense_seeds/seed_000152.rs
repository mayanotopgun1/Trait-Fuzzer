#![allow(warnings)]

trait Bar<'a> {
    fn bar(&self, x: &str, y: &'a str);
}

impl<'a> Bar<'a> for &'a str {
    fn bar(&self, _x: &str, _y: &'a str) {}
}

trait Foo<T> {}

fn main() {
}