#![allow(unused_variables)]

mod m {
    pub struct LooksLikeExternCrate;

    trait LooksTrait {}
    impl LooksTrait for LooksLikeExternCrate {}
}

fn main() {
    let s = m::LooksLikeExternCrate {};
}