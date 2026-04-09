#![feature(ptr_metadata, impl_trait_in_assoc_type)]

use std::ptr::{DynMetadata, Pointee};

trait Trait<U> {}

struct MyDst<T: ?Sized>(T);

trait MetaIs<T: ?Sized, U> {
    type Out;
    fn meta_is() -> Self::Out;
}

impl<T: Pointee<Metadata = U> + ?Sized, U> MetaIs<T, U> for () {
    type Out = impl core::fmt::Debug;
    fn meta_is() -> Self::Out {}
}

fn works<T>() -> impl Fn() {
    move || {
        <() as MetaIs<T, ()>>::meta_is();
        <() as MetaIs<[T], usize>>::meta_is();
        <() as MetaIs<str, usize>>::meta_is();
        <() as MetaIs<dyn Trait<T>, DynMetadata<dyn Trait<T>>>>::meta_is();
        <() as MetaIs<MyDst<T>, ()>>::meta_is();
        <() as MetaIs<((((([u8],),),),),), usize>>::meta_is();
    }
}

fn main() {
    let f = works::<i32>();
    f();
}