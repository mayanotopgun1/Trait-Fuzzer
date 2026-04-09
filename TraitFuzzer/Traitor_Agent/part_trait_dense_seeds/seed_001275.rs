#![feature(return_position_impl_trait_in_trait, inherent_associated_types)]

trait Generator {
    type Item;
    fn next(&self) -> impl core::fmt::Debug;
}

struct D<T> {
    a: T,
}

impl<T: Default + core::fmt::Debug> Generator for D<T> {
    type Item = T;

    fn next(&self) -> impl core::fmt::Debug {
        Self::Item::default()
    }
}

fn main() {}