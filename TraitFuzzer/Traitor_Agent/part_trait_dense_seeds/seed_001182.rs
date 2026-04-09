#![feature(impl_trait_in_assoc_type)]
#![allow(unused_variables)]

mod m {
    pub struct LooksLikeExternCrate;

    pub trait LooksTrait {}
    impl LooksTrait for LooksLikeExternCrate {}


    pub fn create_instance() -> impl LooksTrait {
        LooksLikeExternCrate {}
    }
}

fn main() {
    let s = m::create_instance();
}