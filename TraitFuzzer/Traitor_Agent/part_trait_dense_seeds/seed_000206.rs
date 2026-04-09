#![feature(return_position_impl_trait_in_trait)]

trait Multiply<T> {
    fn multiply(self, other: T) -> impl core::fmt::Debug;
}

impl Multiply<u8> for u8 {
    fn multiply(self, other: u8) -> impl core::fmt::Debug {
        (self as u16) * (other as u16)
    }
}

fn main() {
    let x = 200u8.multiply(4);
    println!("{:?}", x); // Ensure the output is printed for debugging
}