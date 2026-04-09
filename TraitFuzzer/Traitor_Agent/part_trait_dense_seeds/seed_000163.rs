#![allow(unused_attributes)]
#![allow(dead_code)]

mod rustrt {
    extern "C" {
        pub fn rust_get_test_int() -> isize;
    }
}

trait TestIntAccess {
    fn get_test_int(&self) -> isize;
}

impl TestIntAccess for () {
    fn get_test_int(&self) -> isize {
        unsafe { rustrt::rust_get_test_int() }
    }
}

pub fn main() {}