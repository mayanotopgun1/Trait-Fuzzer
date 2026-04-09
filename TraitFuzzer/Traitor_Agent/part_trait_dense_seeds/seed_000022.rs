#![feature(return_position_impl_trait_in_trait)]

mod m {
    pub struct S(u8);

    trait SZ {
        fn value(&self) -> impl core::fmt::Debug;
    }

    impl SZ for S {
        fn value(&self) -> impl core::fmt::Debug {
            self.0
        }
    }

    use S as Z;
}

use m::*;

fn main() {}