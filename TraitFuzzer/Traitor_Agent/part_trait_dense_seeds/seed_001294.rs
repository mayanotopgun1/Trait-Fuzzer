#![feature(return_position_impl_trait_in_trait)]
#![allow(unused_must_use)]

trait IteratorExt: Iterator { }

impl<T> IteratorExt for T where T: Iterator {}

fn bug<T>() -> impl Iterator<Item = [(); { |x: u32| { x }; 4 }]> {
    std::iter::empty()
}

fn ok<T>() -> Box<dyn IteratorExt<Item = [(); { |x: u32| { x }; 4 }]>> {
    Box::new(std::iter::empty())
}

fn main() {
    for _item in ok::<u32>() {}
    for _item in bug::<u32>() {}
}