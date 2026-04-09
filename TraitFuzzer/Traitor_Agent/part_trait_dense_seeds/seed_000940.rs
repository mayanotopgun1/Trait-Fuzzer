#![feature(return_position_impl_trait_in_trait)]
#![allow(incomplete_features)]
#![deny(single_use_lifetimes)]

struct Foo<T>(T);

trait Example {
    type Assoc;
}

impl<'a> Example for Foo<fn(&'a ())> {
    type Assoc = &'a ();
}

trait Other {}
impl Other for u32 {}

trait BarTrait {
    fn bar(_: for<'a> fn(<Foo<fn(&'a ())> as Example>::Assoc)) -> impl Other;
}

impl BarTrait for () {
    fn bar(_: for<'a> fn(<Foo<fn(&'a ())> as Example>::Assoc)) -> impl Other { 0u32 }
}

fn main() {}