#![feature(impl_trait_in_assoc_type)]

trait MaxValue {
    type Output: Copy + PartialOrd;
    const MAX: Self::Output;
}

impl MaxValue for u8 {
    type Output = u8;
    const MAX: u8 = std::u8::MAX;
}

fn main() {
    let _ = <u8 as MaxValue>::MAX;
}