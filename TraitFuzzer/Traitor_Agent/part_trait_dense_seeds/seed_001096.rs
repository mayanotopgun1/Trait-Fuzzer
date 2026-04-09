#![allow(dead_code)]
#![warn(unused_braces)]

use std::cmp::Ordering;

trait PtrCmp<T: ?Sized> {
    fn ptr_cmp(&self, other: &Self) -> Ordering;
}

impl<T: ?Sized> PtrCmp<T> for *const T {
    fn ptr_cmp(&self, other: &Self) -> Ordering {
        self.cmp(other)
    }
}

fn main() {}