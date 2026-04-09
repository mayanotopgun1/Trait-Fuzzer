#![feature(return_position_impl_trait_in_trait)]
#![feature(ptr_metadata)]

trait Bar: Sized + 'static { }

struct Foo<B: Bar> {
    marker: std::marker::PhantomData<B>,
}

trait MetadataOps<T: ?Sized> {
    fn foo(value: &T) -> impl Fn(&T);
}

impl<B: Bar, T: ?Sized> MetadataOps<T> for Foo<B> {
    fn foo(_: &T) -> impl Fn(&T) {
        move |_| ()
    }
}

pub fn main() {}