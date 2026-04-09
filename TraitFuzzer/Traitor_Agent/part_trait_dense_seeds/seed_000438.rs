#![feature(type_alias_impl_trait)]
#![allow(dead_code)]

struct Foo<T>(T);

trait FooTrait<'a, T> {
    type Output;
    fn foo(&self) -> Self::Output;
}

impl<'a, T> FooTrait<'a, &'a T> for Foo<&'a T> {
    type Output = ();
    fn foo(&self) -> Self::Output {}
}

impl<'a, T> FooTrait<'a, &'a mut T> for Foo<&'a mut T> {
    type Output = ();
    fn foo(&self) -> Self::Output {}
}

trait ExtFooTrait<'a, T>: FooTrait<'a, T> {
    fn extended_foo(&self);
}

impl<'a, T> ExtFooTrait<'a, T> for Foo<T>
where
    Self: FooTrait<'a, T>,
{
    fn extended_foo(&self) {}
}

fn main() {}