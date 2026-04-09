#![feature(type_alias_impl_trait)]
#![allow(unused_imports)]

trait ModTrait {}

pub mod a {
    pub trait ModTraitA {}

    impl ModTraitA for () {}
}

pub mod b {
    pub trait ModTraitB {
        fn some_method(&self);
    }

    struct Hidden<T>(T);

    impl<T: core::fmt::Debug> ModTraitB for Hidden<T> {
        fn some_method(&self) {
            println!("{:?}", self.0);
        }
    }
}

use a::*;

trait ExtModTraitA: ModTraitA {
    fn extended_method(&self) {}
}

impl<T: ModTraitA> ExtModTraitA for T {}

fn main() {
}