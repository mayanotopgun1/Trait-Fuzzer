#![allow(dead_code, mismatched_lifetime_syntaxes)]

struct Foo<'a>(&'a u8);

trait FooTrait<'a> {
    fn new(x: &'a u8) -> Self;
}

impl<'a> FooTrait<'a> for Foo<'a> {
    fn new(x: &'a u8) -> Self {
        Foo(x)
    }
}

fn foo(x: &u8) -> Foo<'_> {
    Foo::new(x)
}

fn foo2(x: &'_ u8) -> Foo<'_> {
    Foo::new(x)
}

fn foo3(x: &'_ u8) -> Foo {
    Foo::new(x)
}

fn foo4(_: Foo<'_>) {}

struct Foo2<'a, 'b> {
    a: &'a u8,
    b: &'b u8,
}

trait Foo2Trait<'a, 'b> {
    fn new(a: &'a u8, b: &'b u8) -> Self;
    fn get_b(&self) -> &'b u8;
}

impl<'a, 'b> Foo2Trait<'a, 'b> for Foo2<'a, 'b> {
    fn new(a: &'a u8, b: &'b u8) -> Self {
        Foo2 { a, b }
    }

    fn get_b(&self) -> &'b u8 {
        self.b
    }
}

fn main() {}