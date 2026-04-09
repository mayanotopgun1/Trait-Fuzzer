#![feature(inherent_associated_types)]
#![allow(incomplete_features)]
#![deny(single_use_lifetimes)]

struct Foo<T>(T);

impl<'a> Foo<fn(&'a ())> {
    type Assoc = &'a ();
}

trait Other {}
impl Other for u32 {}

trait BarTrait {
    fn bar(_: for<'a> fn(Foo<fn(&'a ())>::Assoc));
}

impl BarTrait for () {
    fn bar(_: for<'a> fn(Foo<fn(&'a ())>::Assoc)) {}
}

fn main() {}