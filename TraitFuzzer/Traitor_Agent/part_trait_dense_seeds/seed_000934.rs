#![feature(generic_associated_types)]

pub trait Negate<T> {
    type Out<'a>;
    fn negate(self) -> Self::Out<'static>;
}

impl Negate<i8> for i32 {
    type Out<'a> = i8;
    fn negate(self) -> Self::Out<'static> { -self as i8 }
}

impl Negate<i16> for i32 {
    type Out<'a> = i16;
    fn negate(self) -> Self::Out<'static> { -self as i16 }
}

impl Negate<i32> for i32 {
    type Out<'a> = i32;
    fn negate(self) -> Self::Out<'static> { -self }
}

impl Negate<i64> for i64 {
    type Out<'a> = i64;
    fn negate(self) -> Self::Out<'static> { -self }
}

impl Negate<isize> for isize {
    type Out<'a> = isize;
    fn negate(self) -> Self::Out<'static> { -self }
}

trait NegateExt<T>: Negate<T> {
    fn negate_ext(self) -> <Self as Negate<T>>::Out<'static>;
}

impl<S, T> NegateExt<T> for S where S: Negate<T> {
    fn negate_ext(self) -> <Self as Negate<T>>::Out<'static> { self.negate() }
}

pub fn main() {
    let a: i32 = 1;
    let a_neg: i8 = <i32 as NegateExt<i8>>::negate_ext(a);
    println!("{}", a_neg);

    let b: i32 = 1;
    let b_neg: i16 = <i32 as NegateExt<i16>>::negate_ext(b);
    println!("{}", b_neg);

    let c: i32 = 1;
    let c_neg: i32 = <i32 as NegateExt<i32>>::negate_ext(c);
    println!("{}", c_neg);

    let d: i64 = 1;
    let d_neg: i64 = <i64 as NegateExt<i64>>::negate_ext(d);
    println!("{}", d_neg);

    let e: isize = 1;
    let e_neg: isize = <isize as NegateExt<isize>>::negate_ext(e);
    println!("{}", e_neg);
}