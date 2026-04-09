#![feature(return_position_impl_trait_in_trait)]
#![warn(unused)]

#[warn(unused_variables)]
#[expect(unused_variables)]

trait Identity { fn identity(&self) -> impl core::fmt::Debug; }
impl Identity for i32 { fn identity(&self) -> impl core::fmt::Debug { *self } }

fn main() {
    let x = 2;
    let _ = x.identity();
}