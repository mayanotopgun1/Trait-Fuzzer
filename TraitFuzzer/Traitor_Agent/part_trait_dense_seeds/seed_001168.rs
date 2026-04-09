#![feature(return_position_impl_trait_in_trait)]

pub fn main() {
    let mut sum: isize = 0;
    first_ten(|i| { println!("main"); println!("{}", i); sum = sum.add(i); });
    println!("sum");
    println!("{}", sum);
    assert_eq!(sum, 45);
}

trait AddExt {
    type Output;
    fn add(self, rhs: Self) -> Self::Output;
}

impl AddExt for isize {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        self + rhs
    }
}

fn first_ten<F>(mut it: F) where F: FnMut(isize) {
    let mut i: isize = 0;
    while i < 10 { println!("first_ten"); it(i); i = i.add(1); }
}