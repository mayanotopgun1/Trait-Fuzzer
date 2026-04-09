#![allow(dead_code)]

extern "C" {
    fn pow(x: f64, y: f64) -> f64;
}

trait PowLike {
    fn pow(&self, exponent: f64) -> f64;
}

impl PowLike for f64 {
    fn pow(&self, exponent: f64) -> f64 {
        unsafe { pow(*self, exponent) }
    }
}

pub fn main() {}