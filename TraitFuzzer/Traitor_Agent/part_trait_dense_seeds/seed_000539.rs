#![feature(generic_associated_types)]
#![deny(single_use_lifetimes)]
#![allow(dead_code)]
#![allow(unused_variables)]

trait C<'a> {
    type Out;
    fn c(&self, x: &'a u32, y: &'a u32) -> Self::Out;
}

impl<'a> C<'a> for () {
    type Out = ();
    fn c(&self, x: &'a u32, y: &'a u32) -> Self::Out {}
}

trait D<'a> {
    type Out;
    fn d(&self, x: (&'a u32, &'a u32)) -> Self::Out;
}

impl<'a> D<'a> for () {
    type Out = ();
    fn d(&self, x: (&'a u32, &'a u32)) -> Self::Out {}
}

fn main() {
    let _: dyn C<'static, Out = ()>;
    let _: dyn D<'static, Out = ()>;
}