#![crate_type = "lib"]
#![cfg_attr(broken, no_core)]

pub struct S {}

trait SExt {
    fn new() -> Self;
}

impl SExt for S {
    fn new() -> Self {
        S {}
    }
}