#![feature(inherent_associated_types)]

trait Generator {
    type Item;
    fn next(&self) -> Self::Item;
}

struct D<T> {
    a: T,
}

impl<T: Default> Generator for D<T> {
    type Item = T;

    fn next(&self) -> Self::Item {
        Self::Item::default()
    }
}

fn main() {}