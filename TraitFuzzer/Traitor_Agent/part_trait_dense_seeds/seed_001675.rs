#![deny(dead_code)]
#![feature(return_position_impl_trait_in_trait)]

trait UInt: Copy + From<u8> {}

impl UInt for u16 {}

trait Int: Copy {
    type Unsigned: UInt;

    fn as_unsigned(self) -> Self::Unsigned;
}

impl Int for i16 {
    type Unsigned = u16;

    fn as_unsigned(self) -> u16 {
        self as _
    }
}

trait PrivFuncTrait<T: Int>: Sized {
    fn priv_func(x: u8, y: T) -> impl std::fmt::Debug where <T as Int>::Unsigned: std::fmt::Debug;
}

impl<U: Int> PrivFuncTrait<U> for U {
    fn priv_func(x: u8, y: U) -> impl std::fmt::Debug where <U as Int>::Unsigned: std::fmt::Debug {
        (U::Unsigned::from(x), y.as_unsigned())
    }
}

pub trait PubFuncTrait: Sized {
    fn pub_func(x: u8, y: i16) -> impl std::fmt::Debug;
}

impl<T> PubFuncTrait for T {
    fn pub_func(x: u8, y: i16) -> impl std::fmt::Debug {
        <i16 as PrivFuncTrait<i16>>::priv_func(x, y)
    }
}

fn main() {}