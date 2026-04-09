#![no_std]
#![feature(return_position_impl_trait_in_trait)]

extern crate std;

mod foo {
    pub trait Test { fn test() -> Option<i32>; }
    impl Test for () { fn test() -> Option<i32> { Some(2) } }

    pub fn test() -> Option<i32> {
        <() as Test>::test()
    }
}

trait UnwrapOptionExt<T> {
    fn unwrap_option(self) -> T;
}

impl<T> UnwrapOptionExt<T> for core::option::Option<T> {
    fn unwrap_option(self) -> T {
        self.unwrap()
    }
}

fn main() {
    let a = core::option::Option::<&str>::Some("foo");
    a.unwrap_option();
    foo::test().unwrap_option();
}