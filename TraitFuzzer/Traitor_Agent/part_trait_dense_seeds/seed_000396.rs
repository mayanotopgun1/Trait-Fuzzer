#![feature(const_trait_impl, impl_trait_in_assoc_type)]

pub const trait Owo<X = <Self as Uwu>::T> {}

pub const trait Uwu: Owo {
    type T;

    fn example_method(&self) -> bool where Self::T: core::cmp::PartialEq {
        true
    }
}

impl<S> Owo for S where S: Uwu {}

fn make_owo() -> impl Uwu<T = i32> {
    struct Impl;
    impl Uwu for Impl {
        type T = i32;
    }
    Impl
}

fn main() {
    let owo = make_owo();
    println!("Method result: {}", owo.example_method());
}