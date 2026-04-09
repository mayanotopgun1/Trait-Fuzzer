#![feature(return_position_impl_trait_in_trait)]
#![allow(dead_code)]

macro_rules! foo {
    ($x:tt) => (trait AliasTrait { fn t(&self) -> impl core::fmt::Debug; } impl AliasTrait for $x<i32> { fn t(&self) -> impl core::fmt::Debug { 42i32 } })
}

foo!(Box);

fn main() {}