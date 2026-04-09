#![feature(new_range_api)]

use std::{iter, range};

trait RangeExt: Iterator {
    fn zip_with<U>(self, other: U) -> iter::Zip<Self, <U as IntoIterator>::IntoIter>
    where
        Self: Sized,
        U: IntoIterator,
    {
        iter::zip(self, other.into_iter())
    }
}

impl<T> RangeExt for T where T: Iterator {}

fn main() {
    let range_a = (0_u32..256).into_iter();
    let range_b = 0_u8..; // Changed to directly use the range syntax
    for (a, b) in range_a.zip_with(range_b) {
        assert_eq!(a, u32::from(b));
    }

    let mut a = range::RangeFrom::from(0_u8..).into_iter();
    let mut b = 0_u8..;
    assert_eq!(a.next(), b.next());
    assert_eq!(a.nth(5), b.nth(5));
    assert_eq!(a.nth(0), b.next());

    let mut a = range::RangeFrom::from(0_u8..).into_iter();
    let mut b = 0_u8..;
    assert_eq!(a.nth(5), b.nth(5));
    assert_eq!(a.nth(0), b.next());
}