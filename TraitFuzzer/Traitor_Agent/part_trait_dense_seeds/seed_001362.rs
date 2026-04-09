#![feature(pattern_types, rustc_attrs)]
#![feature(pattern_type_macro)]
#![allow(incomplete_features)]

use std::pat::pattern_type;

type X = std::num::NonZero<u32>;
type Y = pattern_type!(u32 is 1..);
type Z = Option<pattern_type!(u32 is 1..)>;
struct NonZeroU32New(pattern_type!(u32 is 1..));

trait Transmute {
    type Target;
    unsafe fn transmute(self) -> Self::Target;
}

impl<'a> Transmute for &'a u32 {
    type Target = Y;
    unsafe fn transmute(self) -> Self::Target {
        std::mem::transmute(*self)
    }
}

impl<'a> Transmute for &'a Y {
    type Target = X;
    unsafe fn transmute(self) -> Self::Target {
        std::mem::transmute(*self)
    }
}

fn main() {
    let x: Y = unsafe { 42_u32.transmute() };
    let z: Z = Some(unsafe { 42_u32.transmute() });
    match z {
        Some(y) => {
            let _: Y = y;
        }
        None => {}
    }
    let x: X = unsafe { (&x).transmute() };
}