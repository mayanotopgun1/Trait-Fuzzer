#![feature(generic_associated_types)]

use std::marker::PhantomData;
use std::fmt::Debug;

trait Family: Sized {
    type Item<'a>;

    fn apply_all<F>(&self, f: F)
    where
        for<'a> F: FnMut(Self::Item<'a>),
    {
    }
}

struct Array<T>(PhantomData<T>);

impl<T: 'static> Family for Array<T> {
    type Item<'a> = &'a T;
}

trait FamilyItemFn<'a, T: Family>: for<'b> Fn(T::Item<'b>) {}

impl<'a, T, F> FamilyItemFn<'a, T> for F
where
    T: Family,
    for<'b> F: Fn(T::Item<'b>),
{
}

trait ArrayExt<T>: for<'a> Family<Item<'a> = &'a T>
where
    Self: Sized + 'static,
{
    fn process_all<F>(&self, f: F)
    where
        for<'a> F: Fn(&'a T),
    {
        self.apply_all(f);
    }
}

impl<T: 'static> ArrayExt<T> for Array<T> {}

trait ArrayDebug<T>: ArrayExt<T>
where
    T: Debug,
{
    fn debug_all(&self) {
        self.process_all(|x| println!("{:?}", x));
    }
}

impl<T: std::fmt::Debug + 'static> ArrayDebug<T> for Array<T> {}

fn process<T: 'static>(array: Array<T>) {
    array.process_all(|_x| {});

    array.process_all(|_x: &T| {});
}

fn main() {}