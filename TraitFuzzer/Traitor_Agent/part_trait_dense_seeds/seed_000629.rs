#![feature(return_position_impl_trait_in_trait)]

trait Foo {
    fn a(&self) -> isize;
    fn b(&self) -> impl FnOnce() -> isize {
        let a = self.a();
        move || a + 2
    }
}

impl Foo for isize {
    fn a(&self) -> isize {
        3
    }
}

trait FooExt: Foo {
    fn c(&self) -> impl FnOnce() -> isize {
        let b = self.b();
        move || b() * 2
    }
}

impl<T: Foo> FooExt for T {}

pub fn main() {
    assert_eq!(3.c()(), 10);
}