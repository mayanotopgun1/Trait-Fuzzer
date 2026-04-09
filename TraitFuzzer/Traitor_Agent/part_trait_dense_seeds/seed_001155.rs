#![feature(ptr_metadata)]

use std::ptr::{DynMetadata, Pointee};

trait Trait<U> {}

struct MyDst<T: ?Sized>(T);

trait MetaIs<T: ?Sized, U> {
    fn meta_is();
}

impl<T: Pointee<Metadata = U> + ?Sized, U> MetaIs<T, U> for () {
    fn meta_is() {}
}

fn works<T>() {
    <() as MetaIs<T, ()>>::meta_is();
    <() as MetaIs<[T], usize>>::meta_is();
    <() as MetaIs<str, usize>>::meta_is();
    <() as MetaIs<dyn Trait<T>, DynMetadata<dyn Trait<T>>>>::meta_is();
    <() as MetaIs<MyDst<T>, ()>>::meta_is();
    <() as MetaIs<((((([u8],),),),),), usize>>::meta_is();
}

fn main() {}