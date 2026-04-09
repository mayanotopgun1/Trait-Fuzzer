#![feature(return_position_impl_trait_in_trait)]

pub trait MathOps<'a> {
    fn compute(&'a self) -> impl core::ops::Mul<Output = f64> + Copy;
}

impl<'a> MathOps<'a> for f64 {
    fn compute(&'a self) -> impl core::ops::Mul<Output = f64> + Copy {
        -self * (self + 2.0 / self) - self * 5.0
    }
}

pub trait ComparisonOps<'a> {
    type Result: std::ops::Not<Output = Self::Result>;
    fn compare(&'a self) -> Self::Result;
}

impl<'a> ComparisonOps<'a> for f64 {
    type Result = bool;
    fn compare(&'a self) -> Self::Result {
        *self == 5.0 || *self < 10.0 || *self <= 2.0 || *self != 22.0 / 7.0 || *self >= 10.0 || *self > 1.0
    }
}

pub trait MathOpsExt<'a>: MathOps<'a> {
    fn compute_squared(&'a self) -> f64 {
        let x = self.compute();
        x * x
    }
}

impl<'a, T> MathOpsExt<'a> for T where T: MathOps<'a> {}

pub trait ComparisonOpsExt<'a>: ComparisonOps<'a> {
    fn compare_negation(&'a self) -> Self::Result {
        !self.compare()
    }
}

impl<'a, T> ComparisonOpsExt<'a> for T where T: ComparisonOps<'a> {}

pub fn main() {
    let pi = 3.1415927f64;
    println!("{}", pi.compute_squared());
    if pi.compare_negation() {
        println!("no");
    }
}