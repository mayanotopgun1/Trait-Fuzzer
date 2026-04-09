#![feature(impl_trait_in_assoc_type)]
#![warn(rust_2021_incompatible_closure_captures)]
#![allow(dropping_references, dropping_copy_types)]

trait DropClosure<T> {
    fn drop_with<F>(&self, f: F) -> impl core::fmt::Debug
    where
        F: FnOnce(&T);
}

impl<T: std::fmt::Debug> DropClosure<T> for T {
    fn drop_with<F>(&self, f: F) -> impl core::fmt::Debug
    where
        F: FnOnce(&T),
    {
        f(self);
        self
    }
}

fn main() {
    let a = "";

    a.drop_with(|_: &&str| drop(a));
}