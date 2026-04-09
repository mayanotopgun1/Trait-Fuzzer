#![allow(dead_code)]
#![feature(impl_trait_in_assoc_type)]

extern "C" {
    fn pow(x: f64, y: f64) -> f64;
}

trait PowLike {
    type Output;
    fn pow(&self, exponent: f64) -> Self::Output;
}

impl PowLike for f64 {
    type Output = impl core::fmt::Debug;
    fn pow(&self, exponent: f64) -> Self::Output {
        unsafe { pow(*self, exponent) }
    }
}

pub fn main() {}