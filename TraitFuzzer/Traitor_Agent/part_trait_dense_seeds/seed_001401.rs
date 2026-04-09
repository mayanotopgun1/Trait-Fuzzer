#![feature(return_position_impl_trait_in_trait)]

extern "C" {

    static TEST1: i32;
    fn test1(i: i32);
}

unsafe extern "C" {
    static TEST2: i32;
    fn test2(i: i32);
}

trait TestTrait {
    unsafe fn call_test(&self, i: i32) -> impl Fn(i32);
}

impl TestTrait for () {
    unsafe fn call_test(&self, i: i32) -> impl Fn(i32) {
        move |x| test1(x + i)
    }
}

trait UnsafeTestTrait {
    unsafe fn call_unsafe_test(&self, i: i32) -> impl Fn(i32);
}

impl UnsafeTestTrait for () {
    unsafe fn call_unsafe_test(&self, i: i32) -> impl Fn(i32) {
        move |x| test2(x + i)
    }
}

fn main() {}