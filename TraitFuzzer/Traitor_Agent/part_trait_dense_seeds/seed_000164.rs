#![feature(return_position_impl_trait_in_trait)]
#![allow(unused_attributes)]
#![allow(dead_code)]

mod rustrt {
    extern "C" {
        pub fn rust_get_test_int() -> isize;
    }
}

trait TestIntAccess {
    fn get_test_int(&self) -> impl core::fmt::Debug;
}

impl TestIntAccess for () {
    fn get_test_int(&self) -> impl core::fmt::Debug {
        unsafe { rustrt::rust_get_test_int() }
    }
}

pub fn main() {}