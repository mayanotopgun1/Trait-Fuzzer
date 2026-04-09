#![allow(unused_allocation)]

trait BoxTrait<T> {
    fn new(value: T) -> Self;
}

impl<T> BoxTrait<T> for Box<T> {
    fn new(value: T) -> Self {
        Box::new(value)
    }
}

trait PartialEqBox<Rhs = Self> {
    fn eq_box(&self, other: &Rhs) -> bool;
}

impl<T: PartialEq> PartialEqBox for Box<T> {
    fn eq_box(&self, other: &Self) -> bool {
        **self == **other
    }
}

trait PartialOrdBox<Rhs = Self>: PartialEqBox<Rhs> {
    fn lt_box(&self, other: &Rhs) -> bool;
    fn le_box(&self, other: &Rhs) -> bool;
    fn gt_box(&self, other: &Rhs) -> bool;
    fn ge_box(&self, other: &Rhs) -> bool;
}

impl<T: PartialOrd> PartialOrdBox for Box<T> {
    fn lt_box(&self, other: &Self) -> bool {
        **self < **other
    }

    fn le_box(&self, other: &Self) -> bool {
        **self <= **other
    }

    fn gt_box(&self, other: &Self) -> bool {
        **self > **other
    }

    fn ge_box(&self, other: &Self) -> bool {
        **self >= **other
    }
}

pub fn main() {
    let i: Box<i32> = BoxTrait::new(100);
    assert!(i.eq_box(&BoxTrait::new(100)));
    assert!(i.lt_box(&BoxTrait::new(101)));
    assert!(i.le_box(&BoxTrait::new(100)));
    assert!(i.gt_box(&BoxTrait::new(99)));
    assert!(i.ge_box(&BoxTrait::new(99)));
}