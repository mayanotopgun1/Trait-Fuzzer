trait Foo {
    fn a(&self) -> isize;
    fn b(&self) -> isize {
        self.a() + 2
    }
}

impl Foo for isize {
    fn a(&self) -> isize {
        3
    }
}

trait FooExt: Foo {
    fn c(&self) -> isize {
        self.b() * 2
    }
}

impl<T: Foo> FooExt for T {}

pub fn main() {
    assert_eq!(3.c(), 10);
}