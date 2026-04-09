#![deny(single_use_lifetimes)]
#![allow(dead_code)]
#![allow(unused_variables)]

trait C<'a> {
    fn c(&self, x: &'a u32, y: &'a u32);
}

impl<'a> C<'a> for () {
    fn c(&self, x: &'a u32, y: &'a u32) {}
}

trait D<'a> {
    fn d(&self, x: (&'a u32, &'a u32));
}

impl<'a> D<'a> for () {
    fn d(&self, x: (&'a u32, &'a u32)) {}
}

fn main() {
    let _: dyn C<'static>;
    let _: dyn D<'static>;
}