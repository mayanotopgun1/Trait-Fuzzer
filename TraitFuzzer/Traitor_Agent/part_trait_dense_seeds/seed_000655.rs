#![allow(dead_code)]

pub trait Foo {
    type Bar;

    fn foo(&self) -> Self;
}

pub struct Static<T:'static>(T);

trait FooExt<F: Foo> where F::Bar : 'static {
    fn new(x: Option<F::Bar>) -> Self;
}

impl<F: Foo> FooExt<F> for Bar<F>
where
    F::Bar : 'static,
{
    fn new(x: Option<F::Bar>) -> Self {
        Bar { x: Static(x) }
    }
}

struct Bar<T:Foo>
    where T::Bar : 'static
{
    x: Static<Option<T::Bar>>
}

impl<T: Foo> Bar<T>
where
    T::Bar: 'static,
{
    fn get_x(&self) -> Option<&T::Bar> {
        self.x.0.as_ref()
    }
}

fn main() { }