#![feature(impl_trait_in_assoc_type)]
#![warn(unused)]

#[warn(unused_variables)]
#[expect(unused_variables)]

trait Identity {
    type Out;
    fn identity(x: Self) -> Self::Out;
}

trait DoubleIdentity: Identity where Self::Out: core::ops::Add<Output = Self::Out> + Copy + Sized {
    fn double_identity(x: Self) -> Self::Out where Self: Sized {
        let v = Self::identity(x);
        v + v
    }
}

impl<T> DoubleIdentity for T where T: Identity, T::Out: core::ops::Add<Output = T::Out> + Copy + Sized {}

impl Identity for i32 {
    type Out = (i32,);
    fn identity(x: Self) -> Self::Out {
        (x,)
    }
}

fn main() {
    let x = 2;
    let _ = Identity::identity(x);
}