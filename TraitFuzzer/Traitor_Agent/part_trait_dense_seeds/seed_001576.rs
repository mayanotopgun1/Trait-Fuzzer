#![feature(return_position_impl_trait_in_trait)]

mod a {
    pub trait ModuleA {
        fn f(&self) -> impl core::fmt::Debug;
        fn g(&self) -> impl core::fmt::Debug;
    }

    impl ModuleA for () {
        fn f(&self) -> impl core::fmt::Debug { 0 }
        fn g(&self) -> impl core::fmt::Debug { 0 }
    }
}

mod b {
    pub use crate::a::*;

    pub struct B;

    impl ModuleA for B {
        fn f(&self) -> impl core::fmt::Debug { "B.f" }
        fn g(&self) -> impl core::fmt::Debug { "B.g" }
    }
}

pub fn main() {
    let b = b::B;
    use a::ModuleA;
    println!("{:?}", b.f());
    println!("{:?}", b.g());
}