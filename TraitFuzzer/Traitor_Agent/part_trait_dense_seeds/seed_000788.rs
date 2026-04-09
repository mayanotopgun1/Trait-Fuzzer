#![feature(return_position_impl_trait_in_trait)]
#![allow(dead_code)]

use std::cell::Cell;

const my_static: u32 = 0;

const LOL: u32 = my_static + 0;

mod my_mod {
    const INSIDE_MOD: u32 = super::my_static + 0;
}

thread_local! {
    static fooFOO: Cell<usize> = unreachable!();
}

trait FooTrait<const foo: u32> {
    fn increment(&self) -> impl std::fmt::Debug;
}

impl<const foo: u32> FooTrait<foo> for () {
    fn increment(&self) -> impl std::fmt::Debug {
        let _a = foo + 1;
        _a
    }
}

fn foo<const foo: u32>() {
    let result = <() as FooTrait<foo>>::increment(&());
    println!("{:?}", result);
}

fn main() {
    let _a = crate::my_static;

    fooFOO.set(9);

    println!("{}", fooFOO.get());
}