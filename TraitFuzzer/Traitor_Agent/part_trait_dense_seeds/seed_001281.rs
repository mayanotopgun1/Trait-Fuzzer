#![feature(if_let_guard)]

#[derive(PartialEq)]
struct Foo {
    x: isize,
}

trait EqFoo {
    fn is_foo(&self) -> bool;
}

impl EqFoo for Foo {
    fn is_foo(&self) -> bool {
        *self == Foo { x: 42 }
    }
}

fn foo(f: Foo) {
    match () {
        () if f.is_foo() => {}
        () if let Foo { x: 0.. } = Foo { x: 42 } => {}
        _ => {}
    }
}

fn main() {}