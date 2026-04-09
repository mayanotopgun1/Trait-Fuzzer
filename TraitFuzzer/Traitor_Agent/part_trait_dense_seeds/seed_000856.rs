#![feature(type_alias_impl_trait)]
#![allow(dead_code)]

trait Borrowable {
    type BorrowType;
    fn borrow(&self) -> Self::BorrowType;
}

impl Borrowable for isize {
    type BorrowType = i32;
    fn borrow(&self) -> Self::BorrowType { *self as i32 }
}

trait BorrowExt: Borrowable {}

impl<T: Borrowable> BorrowExt for T {}

fn borrow<T: BorrowExt>(_v: &T) {}

fn borrow_from_arg_imm_ref(v: Box<isize>) {
    v.borrow();
}

fn borrow_from_arg_mut_ref(v: &mut Box<isize>) {
    v.borrow();
}

fn borrow_from_arg_copy(v: Box<isize>) -> impl Borrowable {
    *v
}

pub fn main() {}