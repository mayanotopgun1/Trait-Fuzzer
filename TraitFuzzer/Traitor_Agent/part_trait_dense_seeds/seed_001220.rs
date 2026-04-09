#![allow(dead_code)]
#![feature(specialization)]

use std::mem::needs_drop;
use std::mem::ManuallyDrop;

struct NeedDrop;

impl Drop for NeedDrop {
    fn drop(&mut self) {}
}

union UnionOk1<T> {
    empty: (),
    value: ManuallyDrop<T>,
}

union UnionOk2 {
    value: ManuallyDrop<NeedDrop>,
}

#[allow(dead_code)]
union UnionOk3<T: Copy> {
    empty: (),
    value: T,
}

trait Foo {}

trait ImpliesCopy: Copy {}

#[allow(dead_code)]
union UnionOk4<T: ImpliesCopy> {
    value: T,
}

trait NeedsDropCheck {
    fn check_needs_drop() -> bool;
}

impl<T> NeedsDropCheck for T {
    default fn check_needs_drop() -> bool {
        needs_drop::<Self>()
    }
}

impl<T: Copy> NeedsDropCheck for T {
    fn check_needs_drop() -> bool {
        false
    }
}

fn main() {
    assert!(!UnionOk1::<NeedDrop>::check_needs_drop());
    assert!(!UnionOk3::<&dyn Foo>::check_needs_drop());
}