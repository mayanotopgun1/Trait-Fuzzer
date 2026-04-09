#![feature(specialization)]
#![allow(non_camel_case_types)]

trait New {
    fn new() -> Self;
}

default impl<T> New for T {
    fn new() -> Self {
        unimplemented!()
    }
}

trait NewPair: New {
    fn new_pair() -> (Self, Self) where Self: Copy {
        let v = Self::new();
        (v, v)
    }
}

impl<T: New + Copy> NewPair for T {}

#[derive(Copy, Clone)]
struct union;

impl New for union {
    fn new() -> Self {
        union { }
    }
}

fn main() {
    let _u = <union as New>::new();
}