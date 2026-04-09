#![feature(specialization)]

macro_rules! test_expr {
    ($expr:expr) => {};
}

macro_rules! test_ty {
    ($a:ty | $b:ty) => {};
}

trait ExprTest {
    fn test(&self);
}

impl<T> ExprTest for T {
    default fn test(&self) {}
}

trait TyTest {
    fn test(&self);
}

impl<T> TyTest for T {
    default fn test(&self) {}
}

fn main() {
    let a = || B;
    let c = C;







    ExprTest::test(&A);
    TyTest::test(&B);






}

struct B;
struct C;
struct D;
struct A;


use std::ops::BitOr;

trait BitwiseOr<Rhs> {
    type Output;
    fn bitor(self, rhs: Rhs) -> Self::Output;
}

impl BitwiseOr<D> for C {
    type Output = Self;

    fn bitor(self, _rhs: D) -> Self::Output {
        self
    }
}