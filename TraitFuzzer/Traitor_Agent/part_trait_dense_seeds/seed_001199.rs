#![allow(unused_imports)]
#![allow(dead_code)]

use std::ops::Deref;
use std::borrow::Borrow as StdBorrow;

pub trait ToOwned: Sized {
    type Owned: StdBorrow<Self>;
    fn to_owned(&self) -> Self::Owned;
}

pub trait Borrow<Borrowed> {
    fn borrow(&self) -> &Borrowed;
}

pub struct Foo<B: ToOwned> {
    owned: B::Owned,
}

trait DerefExt<B: ToOwned>: Deref<Target = B::Owned> {}

impl<B: ToOwned, T: Deref<Target = B::Owned>> DerefExt<B> for T {}

fn foo<B: ToOwned>(this: &Foo<B>) -> &B {
    this.owned.borrow()
}

fn main() {}