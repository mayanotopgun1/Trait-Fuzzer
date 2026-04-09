#![feature(specialization)]

trait AssertEq { fn assert_equal(&self, other: &Self) -> bool; }

default impl<T> AssertEq for T {
    fn assert_equal(&self, other: &Self) -> bool {
        false
    }
}

impl<T> AssertEq for T where T: PartialEq {
    fn assert_equal(&self, other: &Self) -> bool {
        self == other
    }
}

fn main() {
    let a = 1;
    let b = 2;
    assert!(!a.assert_equal(&b));
}