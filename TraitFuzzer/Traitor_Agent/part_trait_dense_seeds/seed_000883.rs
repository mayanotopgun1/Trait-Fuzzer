#![allow(dead_code)]
#![allow(unreachable_code)]

use std::ops::Add;

trait AddWithEarlyReturn {
    type Output;
    fn add_with_early_return(&self) -> Self::Output;
}

impl<T: Add<Output = T> + Copy> AddWithEarlyReturn for T {
    type Output = T;
    fn add_with_early_return(&self) -> Self::Output {
        *self + { return *self }
    }
}

pub fn main() {

    let _result1 = 42i32.add_with_early_return();
    let _result2 = 3.14f64.add_with_early_return();
}