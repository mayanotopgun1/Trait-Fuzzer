#[derive(PartialEq, Debug)]
struct Foo(isize);

#[derive(PartialEq, Debug)]
struct Bar(isize, isize);

trait MakeFoo {
    fn make_foo(x: isize) -> Self;
}

impl MakeFoo for Foo {
    fn make_foo(x: isize) -> Self {
        Foo(x)
    }
}

trait MakeBar {
    fn make_bar(x: isize, y: isize) -> Self;
}

impl MakeBar for Bar {
    fn make_bar(x: isize, y: isize) -> Self {
        Bar(x, y)
    }
}

pub fn main() {
    let f = Foo::make_foo;
    let g = Bar::make_bar;
    assert_eq!(f(42), Foo(42));
    assert_eq!(g(4, 7), Bar(4, 7));
}