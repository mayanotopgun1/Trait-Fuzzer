#![allow(dead_code)]

struct Foo<T>(T);

trait FooTrait<'a, T> {
    fn foo(&self);
}

impl<'a, T> FooTrait<'a, &'a T> for Foo<&'a T> {
    fn foo(&self) {}
}

impl<'a, T> FooTrait<'a, &'a mut T> for Foo<&'a mut T> {
    fn foo(&self) {}
}

trait ExtFooTrait<'a, T>: FooTrait<'a, T> {
    fn extended_foo(&self);
}

impl<'a, T> ExtFooTrait<'a, T> for Foo<T> where Self: FooTrait<'a, T> {
    fn extended_foo(&self) {}
}

fn main() {}