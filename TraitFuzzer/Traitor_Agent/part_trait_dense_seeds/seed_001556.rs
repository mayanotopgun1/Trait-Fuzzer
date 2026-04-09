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

fn main() {}