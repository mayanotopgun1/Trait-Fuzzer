#![feature(return_position_impl_trait_in_trait)]

trait Iterable {
    type Item<'a>
    where
        Self: 'a;

    fn iter<'a>(&'a self) -> impl Iterator<Item = Self::Item<'a>> + 'a;
}

trait IterableExt: Iterable {
    fn first_item<'a>(&'a self) -> Option<Self::Item<'a>> {
        self.iter().next()
    }
}

impl<T> Iterable for [T] {
    type Item<'a> = &'a T where T: 'a;

    fn iter<'a>(&'a self) -> impl Iterator<Item = Self::Item<'a>> + 'a {
        self.iter()
    }
}

impl<T: Iterable + ?Sized> IterableExt for T {}

fn main() {
    let v = vec![1, 2, 3];

    assert_eq!(Some(&1), v.first_item());
}