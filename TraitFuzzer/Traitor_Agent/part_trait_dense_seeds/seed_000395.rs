#![feature(const_trait_impl)]

pub const trait Owo<X = <Self as Uwu>::T> {}

pub const trait Uwu: Owo {
    type T;

    fn example_method(&self) -> bool where Self::T: core::cmp::PartialEq {
        true
    }
}

impl<S> Owo for S where S: Uwu {}

fn main() {}