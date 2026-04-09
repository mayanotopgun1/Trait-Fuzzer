#![crate_type = "lib"]
#![feature(transmutability, type_alias_impl_trait)]
#![allow(dead_code)]

mod assert {
    use std::mem::{Assume, TransmuteFrom};

    pub trait AssertTransmutable<Src> {
        fn is_transmutable();
    }

    impl<Dst: ?Sized, Src> AssertTransmutable<Src> for Dst
    where
        Dst: TransmuteFrom<Src, { Assume::SAFETY }>,
    {
        fn is_transmutable() {}
    }
}

trait TestTrait {
    fn test_fn();
}

impl TestTrait for () {
    fn test_fn() {
        type Src = ();
        #[repr(C)]
        struct Dst;

        type HiddenAssertion = Dst;
        <HiddenAssertion as assert::AssertTransmutable<Src>>::is_transmutable();
    }
}

fn main() {
    <() as TestTrait>::test_fn(); // Use the fully-qualified path to specify the impl type
}