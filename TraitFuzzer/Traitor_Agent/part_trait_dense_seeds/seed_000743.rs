#![allow(dead_code)]
#![allow(non_camel_case_types)]

pub enum void {}

trait VoidTrait {
    // Define any associated methods or constants here if needed
}

impl VoidTrait for void {}

mod bindgen {
    use super::{void, VoidTrait};

    extern "C" {
        pub fn printf(v: void);
    }
}

pub fn main() {}