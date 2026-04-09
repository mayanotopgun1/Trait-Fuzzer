#![feature(return_position_impl_trait_in_trait)]

struct T {}

trait TraitT {
    fn method(&self) -> impl core::fmt::Debug;
}

impl TraitT for T {
    fn method(&self) -> impl core::fmt::Debug {
        "Method called"
    }
}

fn main() {
    let t = T {};
    println!("{:?}", t.method());
}