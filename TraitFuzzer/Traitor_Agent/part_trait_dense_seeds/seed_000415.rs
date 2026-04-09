#![feature(ptr_metadata)]

trait Bar: Sized + 'static { }

struct Foo<B: Bar> {
    marker: std::marker::PhantomData<B>,
}

trait MetadataOps<T: ?Sized> {
    fn foo(value: &T);
}

impl<B: Bar, T: ?Sized> MetadataOps<T> for Foo<B> {
    fn foo(value: &T) {
        std::ptr::metadata(value);
    }
}

pub fn main() {}