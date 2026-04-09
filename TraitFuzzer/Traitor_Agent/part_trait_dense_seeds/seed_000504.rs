#![feature(impl_trait_in_assoc_type)]

macro_rules! as_stmt { ($s:stmt) => { $s }; }

trait Declare {
    fn declare(&self) -> impl core::fmt::Debug;
}

impl Declare for () {
    fn declare(&self) -> impl core::fmt::Debug {
        let _x = 0u32;
        _x
    }
}

fn main() {
    as_stmt!(println!("{:?}", ().declare()));
}