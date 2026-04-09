#![feature(specialization)]

enum Inner { Member(u32) }

trait CreateInner {
    fn create_member(value: u32) -> Self;
}

default impl<T> CreateInner for T {
    default fn create_member(_value: u32) -> Self {
        panic!("Default implementation should not be used")
    }
}

impl CreateInner for Inner {
    fn create_member(value: u32) -> Self {
        Inner::Member(value)
    }
}

fn main() {
    let _ = Inner::create_member(0);
}