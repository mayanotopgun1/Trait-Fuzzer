#![feature(unboxed_closures)]

#![allow(dead_code, unused_variables)]

use std::marker::PhantomData;

struct Type<'a> {
    data: PhantomData<fn(&'a u32) -> &'a u32>,
}

trait FooType<'a> {
    fn foo(&self) -> Type<'a>;
}

impl<'a> FooType<'a> for () {
    fn foo(&self) -> Type<'a> {
        loop {}
    }
}

trait BarClone: Clone {}

impl<T> BarClone for T where T: Clone {}

fn bar<T, U>(t: T, x: U) -> U
where
    T: FnOnce(U) -> U,
{
    t(x)
}

#[cfg(ok)]
fn baz<'a, 'b>(x: Type<'a>, y: Type<'b>) -> impl Iterator<Item = (Type<'a>, Type<'b>)>
where
    Type<'a>: BarClone,
    Type<'b>: BarClone,
{
    let a = bar::<_, Type<'a>>(|_| x.clone(), ());
    let b = bar::<_, Type<'b>>(|_| y.clone(), ());
    std::iter::once((a, b))
}

#[cfg(oneuse)]
fn baz<'a, 'b>(x: Type<'a>, y: Type<'b>) -> impl Iterator<Item = (Type<'a>, Type<'b>)>
where
    Type<'a>: BarClone,
    Type<'b>: BarClone,
{
    let f = || x.clone();
    let a = bar(f, y);

    let g = || y.clone();
    let b = bar(g, x);

    std::iter::once((a, b))
}

#[cfg(transmute)]
fn baz<'a>(x: Type<'a>) -> impl FooType<'static>
where
    Type<'a>: BarClone,
{
    bar::<_, Type<'static>>(|_| Type { data: PhantomData }, x)
}

#[cfg(krisskross)]
fn transmute<'a, 'b>(x: Type<'a>, y: Type<'b>) -> impl Iterator<Item = (Type<'a>, Type<'b>)>
where
    Type<'a>: BarClone,
    Type<'b>: BarClone,
{
    let a = bar::<_, Type<'a>>(|_| x.clone(), ());
    let b = bar::<_, Type<'b>>(|_| y.clone(), ());
    std::iter::once((a, b))
}

fn main() {}