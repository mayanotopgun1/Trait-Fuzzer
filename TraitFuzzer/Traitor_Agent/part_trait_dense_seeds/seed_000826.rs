#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

trait Bind<const N: usize> { fn bind(self) -> [u8; N]; }
impl<const N: usize> Bind<N> for [u8; N] { fn bind(self) -> [u8; N] { self } }

trait Sink<const N: usize> { fn sink(self); }
impl<const N: usize> Sink<N> for [u8; N] { fn sink(self) {} }

fn main() {
    let mut arr: [u8; 5] = Default::default();
    arr = arr.bind();
    arr.sink();
}