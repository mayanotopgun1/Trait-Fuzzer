#![allow(non_camel_case_types)]

#[derive(Copy, Clone)]
struct I { i: isize }

trait TestRec {
    fn test(&self) -> I;
}

impl TestRec for bool {
    fn test(&self) -> I {
        if *self { I {i: 100} } else { I {i: 101} }
    }
}

fn test_rec() {
    let rs = true.test();
    assert_eq!(rs.i, 100);
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum mood { happy, sad, }

trait PartialEqExt {
    fn eq_usize(&self, other: &Self) -> bool;
    fn ne_usize(&self, other: &Self) -> bool;
}

impl PartialEqExt for mood {
    fn eq_usize(&self, other: &Self) -> bool {
        (*self as isize) == (*other as isize)
    }
    fn ne_usize(&self, other: &Self) -> bool {
        !(*self).eq_usize(other)
    }
}

fn test_tag() {
    let rs = if true { mood::happy } else { mood::sad };
    assert_eq!(rs, mood::happy);
}

pub fn main() { test_rec(); test_tag(); }