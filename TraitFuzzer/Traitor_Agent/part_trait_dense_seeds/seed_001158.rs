#![feature(specialization)]

use std::marker;

pub struct Foo<T>(marker::PhantomData<T>);

trait IteratorExt: Iterator {
    fn next_opt(&mut self) -> Option<Self::Item> {
        self.next()
    }
}

default impl<I> IteratorExt for I where I: Iterator {}

impl<T> IteratorExt for Foo<T> {}

impl<T> Iterator for Foo<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        None
    }
}

trait DropExt: Drop {
    fn drop_with_next(&mut self) {

        drop(self);
    }
}

default impl<D> DropExt for D where D: Drop {}

impl<T> DropExt for Foo<T> {}

impl<T> Drop for Foo<T> {
    fn drop(&mut self) {
        self.next_opt();
    }
}

pub fn foo<'a>(_: Foo<&'a ()>) {}

pub fn main() {}