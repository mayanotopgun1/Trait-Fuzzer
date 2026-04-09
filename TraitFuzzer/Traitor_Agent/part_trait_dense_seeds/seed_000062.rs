#![allow(dead_code)]
#![allow(unused_unsafe)]

#![feature(generic_associated_types, core_intrinsics, rustc_attrs)]

use std::mem;

trait Debuggable {
    type Out;
    fn debug(&self) -> Self::Out;
}

#[derive(Debug)]
struct Inner {
    c64: u32
}

impl Debuggable for Inner {
    type Out = String;
    fn debug(&self) -> Self::Out {
        format!("{:?}", self)
    }
}

#[derive(Debug)]
struct Outer {
    c8: u8,
    t: Inner
}

impl Debuggable for Outer {
    type Out = String;
    fn debug(&self) -> Self::Out {
        format!("{:?}", self)
    }
}

mod m {
    pub fn align() -> usize { 4 }
    pub fn size() -> usize { 8 }
}

pub fn main() {
    unsafe {
        let x = Outer {c8: 22, t: Inner {c64: 44}};

        let y = x.debug();

        println!("align inner = {:?}", mem::align_of::<Inner>());
        println!("size outer = {:?}", mem::size_of::<Outer>());
        println!("y = {:?}", y);

        assert_eq!(mem::align_of::<Inner>(), m::align());

        assert_eq!(mem::size_of::<Outer>(), m::size());

        assert_eq!(y, "Outer { c8: 22, t: Inner { c64: 44 } }".to_string());
    }
}