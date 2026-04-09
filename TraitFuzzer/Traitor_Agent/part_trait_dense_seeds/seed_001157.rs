use std::marker;

pub struct Foo<T>(marker::PhantomData<T>);

trait IteratorExt: Iterator {
    fn next_opt(&mut self) -> Option<Self::Item> {
        self.next()
    }
}

impl<T> IteratorExt for Foo<T> {}

impl<T> Iterator for Foo<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        None
    }
}

trait DropExt: Drop {
    fn drop_with_next(&mut self) {
        // Use the `drop` function instead of explicit destructor call
        drop(self);
    }
}

impl<T> DropExt for Foo<T> {}

impl<T> Drop for Foo<T> {
    fn drop(&mut self) {
        self.next_opt();
    }
}

pub fn foo<'a>(_: Foo<&'a ()>) {}

pub fn main() {}