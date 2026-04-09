#![allow(stable_features)]
#![feature(iter_arith, impl_trait_in_assoc_type)]

trait Summable {
    type Iter<'a>: Iterator<Item = &'a u64>;
    fn sum_slice(&self) -> u64;
}

impl Summable for [u64; 3] {
    type Iter<'a> = std::slice::Iter<'a, u64>;

    fn sum_slice(&self) -> u64 {
        self.iter().sum()
    }
}

fn main() {
    let x: [u64; 3] = [1, 2, 3];
    assert_eq!(6, x.sum_slice());
}