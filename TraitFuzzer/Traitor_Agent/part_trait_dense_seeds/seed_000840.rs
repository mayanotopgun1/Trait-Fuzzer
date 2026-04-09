#![warn(rust_2021_incompatible_closure_captures)]
#![allow(dropping_references, dropping_copy_types)]

trait DropClosure<T> {
    fn drop_with<F>(&self, f: F)
    where
        F: FnOnce(&T);
}

impl<T> DropClosure<T> for T {
    fn drop_with<F>(&self, f: F)
    where
        F: FnOnce(&T),
    {
        f(self);
    }
}

fn main() {
    if let a = "" {

        a.drop_with(|_: &&str| drop(a));
    }
}