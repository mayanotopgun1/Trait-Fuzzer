#![warn(deref_into_dyn_supertrait)]
#![feature(impl_trait_in_assoc_type)]

use std::ops::Deref;

trait Bar<T> {}

trait Foo: Bar<i32> {
    fn as_dyn_bar_u32<'a>(&self) -> &(dyn Bar<u32> + 'a);
}

impl<'a> Deref for dyn Foo + 'a {

    type Target = dyn Bar<u32> + 'a;

    fn deref(&self) -> &Self::Target {
        self.as_dyn_bar_u32()
    }
}

trait FooExt: Foo {
    fn as_static_str(&self) -> &'static str where Self: Sized {
        "FooExt"
    }
}

impl<T: Foo> FooExt for T {}

fn main() {}