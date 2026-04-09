#![feature(derive_coerce_pointee)]
#![feature(arbitrary_self_types)]
#![feature(return_position_impl_trait_in_trait)]

use std::ops::Deref;
use std::marker::CoercePointee;
use std::sync::Arc;

trait MyTrait<T> {}

#[derive(CoercePointee)]
#[repr(transparent)]
struct MyArc<T: ?Sized + MyTrait<u8>>(Arc<T>);

impl<T: ?Sized + MyTrait<u8>> Deref for MyArc<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

trait Mirror {
    type Assoc;
}
impl<T> Mirror for T {
    type Assoc = T;
}

trait MyOtherTrait: MyTrait<<u8 as Mirror>::Assoc> {
    fn foo(self: MyArc<Self>);
}

trait MyOtherTraitExt: MyOtherTrait + MyTrait<<u8 as Mirror>::Assoc> {
    fn bar(self: MyArc<Self>) -> impl core::fmt::Debug {}
}
impl<T> MyOtherTraitExt for T where T: MyOtherTrait + MyTrait<<u8 as Mirror>::Assoc> {}

fn test(_: MyArc<dyn MyOtherTrait>) {}

fn main() {}