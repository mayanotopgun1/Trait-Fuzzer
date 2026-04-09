#![crate_type = "lib"]
#![feature(transmutability, specialization)]
#![allow(dead_code)]

mod assert {
    use std::mem::{Assume, TransmuteFrom};

    pub trait AssertTransmutable<Src> {
        fn is_transmutable() where Self: TransmuteFrom<Src, { Assume::SAFETY }>;
    }

    impl<T, Src> AssertTransmutable<Src> for T {
        default fn is_transmutable() where Self: TransmuteFrom<Src, { Assume::SAFETY }> {}
    }
}

fn test() {
    #[repr(C)]
    struct Src;
    type Dst = ();
    <Dst as assert::AssertTransmutable<Src>>::is_transmutable();
}