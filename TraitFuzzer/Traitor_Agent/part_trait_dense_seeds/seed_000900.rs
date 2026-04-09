#![feature(return_position_impl_trait_in_trait)]
#![deny(unused_qualifications)]

trait Defaultable {
    fn default_value() -> impl core::fmt::Debug;
}

impl Defaultable for u64 {
    fn default_value() -> impl core::fmt::Debug {
        u64::default()
    }
}

pub fn bar() -> impl core::fmt::Debug {
    u64::default_value()
}

fn main() {}