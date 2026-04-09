#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ptr;

trait FooTrait {
    fn new(x: usize) -> impl FooTrait;
}

impl FooTrait for foo {
    fn new(x: usize) -> impl FooTrait {
        foo(X { x, nxt: ptr::null() })
    }
}

struct foo(X);

struct X {
    x: usize,
    nxt: *const foo,
}

pub fn main() {
    let _x = foo::new(0);
}