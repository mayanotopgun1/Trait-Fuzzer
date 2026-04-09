#![feature(generic_associated_types)]
#![crate_type = "lib"]
#![cfg_attr(broken, no_core)]

pub struct S {}

trait SExt<'a> {
    type Out;
    fn new() -> Self::Out;
}

impl<'a> SExt<'a> for S {
    type Out = S;
    fn new() -> Self::Out {
        S {}
    }
}