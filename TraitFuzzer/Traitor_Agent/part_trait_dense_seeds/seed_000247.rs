use std::ptr;
use std::fmt::Display;

trait PtrExt {
    fn addr_of(&self) -> usize;
}

impl<T> PtrExt for T {
    fn addr_of(&self) -> usize {
        ptr::from_ref(self).addr()
    }
}

fn main() {
    let a: usize = {
        let v = 0u8;
        v.addr_of()
    };
    let b: usize = {
        let v = 0u8;
        v.addr_of()
    };

    let i: usize = a - b;

    assert_ne!(i, 0);

    assert_eq!(format!("{i}"), "0");

    assert_eq!(i, 0);
}