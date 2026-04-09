#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

pub trait IsTrue<const T: bool> {}
impl IsTrue<true> for () {}

pub trait IsZST {}

pub trait IsZSTExt: IsZST {
    fn is_zst(&self) -> bool;
}

impl<T> IsZSTExt for T
where
    (): IsTrue<{ std::mem::size_of::<T>() == 0 }>,
{
    fn is_zst(&self) -> bool {
        true
    }
}

impl<T> IsZST for T
where
    (): IsTrue<{ std::mem::size_of::<T>() == 0 }>,
{}

fn _func() -> impl IsZSTExt {
    || {}
}

fn main() {
    let zst = _func();
    println!("Is ZST: {}", zst.is_zst());
}