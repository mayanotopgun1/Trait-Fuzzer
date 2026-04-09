#![feature(with_negative_coherence)]

use std::ops::DerefMut;

trait Foo {}
impl<T: DerefMut> Foo for T {}
impl<U> Foo for &U {}

trait Bar {
    fn bar(&self);
}

impl<T: Foo> Bar for T {
    fn bar(&self) {}
}

fn main() {}