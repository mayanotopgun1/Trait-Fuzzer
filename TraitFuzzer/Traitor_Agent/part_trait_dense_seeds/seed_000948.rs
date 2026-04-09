#![allow(dead_code)]
#![allow(deprecated)]

use std::mem;

#[derive(PartialEq, Debug)]
enum Foo {
    A(u32),
    Bar([u16; 4]),
    C,
}

trait FooProperties {
    fn is_c(&self) -> bool;
    fn size_of(&self) -> usize;
    fn align_of(&self) -> usize;
}

impl FooProperties for Foo {
    fn is_c(&self) -> bool {
        matches!(self, Foo::C)
    }

    fn size_of(&self) -> usize {
        mem::size_of::<Foo>()
    }

    fn align_of(&self) -> usize {
        mem::align_of::<Foo>()
    }
}

static FOO: Foo = Foo::C;

fn main() {
    assert_eq!(FOO.is_c(), true);
    assert_eq!(FOO.size_of(), 12);
    assert_eq!(FOO.align_of(), 4);
}