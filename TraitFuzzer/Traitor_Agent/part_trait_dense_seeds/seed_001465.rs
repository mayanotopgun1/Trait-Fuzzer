#![allow(arithmetic_overflow)]

trait AddExt {
    type Output;
    fn add_ext(self, rhs: Self) -> Self::Output;
}

impl AddExt for u8 {
    type Output = u8;
    fn add_ext(self, rhs: Self) -> Self::Output {
        self + rhs
    }
}

fn main() {
    let _x = 200u8.add_ext(200u8).add_ext(200u8);
}