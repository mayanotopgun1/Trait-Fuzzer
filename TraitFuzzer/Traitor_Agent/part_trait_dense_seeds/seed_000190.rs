#![feature(sized_hierarchy)]
use std::marker::MetaSized;

type DynFoo = dyn Foo;
type DynMetaSized = dyn MetaSized;

trait Foo: std::fmt::Debug + MetaSized {
    fn foo_debug(&self) -> String {
        format!("{:?}", self)
    }
}

impl<T: std::fmt::Debug + MetaSized> Foo for T {}

fn unsize_sized<T: 'static>(x: Box<T>) -> Box<DynMetaSized> {
    x
}

fn unsize_subtrait(x: Box<DynFoo>) -> Box<DynMetaSized> {
    x
}

fn main() {
    let _bx = unsize_sized(Box::new(vec![1, 2, 3]));

    let bx: Box<DynFoo> = Box::new(vec![1, 2, 3]);
    let _ = bx.foo_debug();
    let _bx = unsize_subtrait(bx);
}