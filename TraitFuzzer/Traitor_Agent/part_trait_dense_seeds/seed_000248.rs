#![feature(return_position_impl_trait_in_trait)]

use std::ptr;
use std::fmt::Display;

trait PtrExt {
    fn addr_of(&self) -> impl Display;
}

impl<T> PtrExt for T {
    fn addr_of(&self) -> impl Display {
        ptr::from_ref(self).addr()
    }
}

fn main() {
    let a: usize = {
        let v = 0u8;
        let x = v.addr_of().to_string();
        x.parse().unwrap()
    };
    let b: usize = {
        let v = 0u8;
        let x = v.addr_of().to_string();
        x.parse().unwrap()
    };

    let i: usize = a - b;

    assert_ne!(i, 0);

    assert_eq!(format!("{i}"), "0");

    assert_eq!(i, 0);
}