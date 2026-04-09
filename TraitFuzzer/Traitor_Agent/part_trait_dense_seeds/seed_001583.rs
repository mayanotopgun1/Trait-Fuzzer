#![allow(dead_code)]

trait MultiDispatch<T> {
    type O;
}

trait DispatchHelper {
    fn dispatch<U, T: Trait<B = U>>(t: &T, u: U) -> <T::A as MultiDispatch<U>>::O where T::A: MultiDispatch<U>;
}

impl<T: Trait<B = i32>> DispatchHelper for T {
    fn dispatch<U, OtherT: Trait<B = U>>(t: &OtherT, u: U) -> <OtherT::A as MultiDispatch<U>>::O where OtherT::A: MultiDispatch<U> {
        OtherT::new(u)
    }
}

trait Trait {
    type A: MultiDispatch<Self::B, O = Self>;
    type B;

    fn new<U>(u: U) -> <Self::A as MultiDispatch<U>>::O where Self::A : MultiDispatch<U>;
}

fn test<T: Trait<B=i32>>(b: i32) -> T where T::A : MultiDispatch<i32> { T::new(b) }

fn main() {}