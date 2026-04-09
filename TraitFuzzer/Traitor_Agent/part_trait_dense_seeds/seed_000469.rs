#![allow(unused_variables)]
#![feature(specialization)]

trait DefaultExt {
    fn create_default() -> Self;
}

default impl<T> DefaultExt for T {
    default fn create_default() -> T {
        panic!("Default implementation not provided")
    }
}

impl DefaultExt for X {
    fn create_default() -> X {
        X { x: 42 }
    }
}

impl<T> DefaultExt for Y<T>
where
    T: Default,
{
    fn create_default() -> Y<T> {
        Y { y: Default::default() }
    }
}

struct X {
    pub x: usize,
}

impl Default for X {
    fn default() -> X {
        Self::create_default()
    }
}

struct Y<T> {
    pub y: T,
}

impl<T: Default> Default for Y<T> {
    fn default() -> Y<T> {
        Self::create_default()
    }
}

fn main() {
    let X { x: _ } = X::create_default();
    let Y { y: X { x } } = Y::<X>::create_default();
}