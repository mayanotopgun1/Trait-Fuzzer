#![allow(non_camel_case_types)]
#![feature(associated_type_defaults)]
#![feature(return_position_impl_trait_in_trait)]

trait clam<A> {
    type Alias;
    fn get(self) -> Self::Alias;
}

trait foo<A>: clam<A> {
    fn bar<B, C>(&self, _c: C) -> impl core::fmt::Debug where C: clam<A>;
}

impl<T, A> foo<A> for T where T: clam<A>, <T as clam<A>>::Alias: core::fmt::Debug {
    fn bar<B, C>(&self, _c: C) -> impl core::fmt::Debug
        where C: clam<A>
    {
        unimplemented!()
    }
}

pub fn main() {}