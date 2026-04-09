#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![feature(specialization)]

trait Call {
    fn call(&self);
}

default impl<T> Call for T {
    default fn call(&self) {}
}

impl Call for S {
    fn call(&self) {
        (self.f)();
    }
}

impl Call for T {
    fn call(&self) {
        (self.f)();
    }
}

struct T { f: extern "Rust" fn() }
struct S { f: extern "Rust" fn() }

fn fooS(t: impl Call) {
    t.call();
}

fn fooT(t: impl Call) {
    t.call();
}

fn bar() {
}

pub fn main() {
    let x: extern "Rust" fn() = bar;
    fooS(S {f: x});
    fooS(S {f: bar});

    let x: extern "Rust" fn() = bar;
    fooT(T {f: x});
    fooT(T {f: bar});
}