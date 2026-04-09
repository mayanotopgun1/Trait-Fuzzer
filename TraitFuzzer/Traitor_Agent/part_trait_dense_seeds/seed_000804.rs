#![feature(impl_trait_in_assoc_type)]

#[derive(PartialEq, Debug)]
struct Foo(isize);

#[derive(PartialEq, Debug)]
struct Bar(isize, isize);

trait MakeFoo {
    type Out;
    fn make_foo(&self, x: isize) -> Self::Out;
}

impl MakeFoo for Foo {
    type Out = Self;
    fn make_foo(&self, x: isize) -> Self::Out {
        Foo(x)
    }
}

trait MakeBar {
    type Out;
    fn make_bar(&self, x: isize, y: isize) -> Self::Out;
}

impl MakeBar for Bar {
    type Out = Self;
    fn make_bar(&self, x: isize, y: isize) -> Self::Out {
        Bar(x, y)
    }
}

pub fn main() {
    let f: &dyn MakeFoo<Out = Foo> = &Foo(0);
    let g: &dyn MakeBar<Out = Bar> = &Bar(0, 0);
    assert_eq!(f.make_foo(42), Foo(42));
    assert_eq!(g.make_bar(4, 7), Bar(4, 7));
}