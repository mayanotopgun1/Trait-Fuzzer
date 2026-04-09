#![allow(unused_assignments)]
#![allow(static_mut_refs)]
#![feature(specialization)]

use std::mem::{ManuallyDrop, drop};

struct S;

union U {
    a: ManuallyDrop<S>
}

trait DropExt {
    fn custom_drop(&mut self);
}

default impl<T> DropExt for T {
    default fn custom_drop(&mut self) {

    }
}

impl Drop for S {
    fn drop(&mut self) {
        unsafe { CHECK += 10; }
    }
}

impl DropExt for S {
    fn custom_drop(&mut self) {
        drop(self);
    }
}

impl Drop for U {
    fn drop(&mut self) {
        unsafe { CHECK += 1; }
    }
}

impl DropExt for U {
    fn custom_drop(&mut self) {
        drop(self);
    }
}

trait CustomDropMap: DropExt {
    fn map_custom_drop(&mut self) where Self: Sized {
        self.custom_drop();
    }
}

impl<T: DropExt> CustomDropMap for T {}

static mut CHECK: u8 = 0;

fn main() {
    unsafe {
        let mut u = U { a: ManuallyDrop::new(S) };
        assert_eq!(CHECK, 0);
        u = U { a: ManuallyDrop::new(S) };
        assert_eq!(CHECK, 1);
        (*u.a).map_custom_drop();
        assert_eq!(CHECK, 11);
    }
}