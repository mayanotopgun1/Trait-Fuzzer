#![feature(type_alias_impl_trait)]
#![feature(impl_trait_in_assoc_type)]
#![allow(dead_code)]

struct Foo<T>(T);

trait FooTrait<'a, T> {
    type Assoc;
    fn foo(&self) -> Self::Assoc;
}

impl<'a, T> FooTrait<'a, &'a T> for Foo<&'a T> {
    type Assoc = impl core::fmt::Debug;
    fn foo(&self) -> Self::Assoc { () }
}
impl<'a, T> FooTrait<'a, &'a mut T> for Foo<&'a mut T> {
    type Assoc = impl core::fmt::Debug;
    fn foo(&self) -> Self::Assoc { () }
}

fn main() {}