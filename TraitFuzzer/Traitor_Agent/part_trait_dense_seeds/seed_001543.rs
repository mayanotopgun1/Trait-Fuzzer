#![crate_type = "lib"]
#![feature(type_alias_impl_trait)]

pub struct Fish {
    pub x: isize
}

trait FishEq {
    fn eq(&self, other: &Self) -> bool;
    fn ne(&self, other: &Self) -> bool;
}

struct HiddenFishEqImpl;

impl FishEq for HiddenFishEqImpl {
    fn eq(&self, _: &Self) -> bool { true }
    fn ne(&self, _: &Self) -> bool { false }
}

type HiddenFishEq = HiddenFishEqImpl;

mod unexported {
    use super::{Fish, FishEq};

    struct HiddenPartialEqImpl;

    impl PartialEq for HiddenPartialEqImpl {
        fn eq(&self, other: &Self) -> bool {
            true
        }

        fn ne(&self, other: &Self) -> bool {
            false
        }
    }

    type HiddenPartialEq = HiddenPartialEqImpl;
}