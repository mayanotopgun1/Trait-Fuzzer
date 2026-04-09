#![feature(return_position_impl_trait_in_trait)]
#![feature(generic_associated_types)]

trait Foo {
    type EarlyIter<'a, T>: Iterator<Item = &'a T> where Self: 'a, T: 'a;
    fn early<'a, T: 'a>(x: &'a T) -> Self::EarlyIter<'a, T>;

    type LateIter<'a, T>: Iterator<Item = &'a T> where Self: 'a, T: 'a;
    fn late<'a, T>(&self, x: &'a T) -> Self::LateIter<'a, T>;
}

trait FooExt: Foo {
    fn extended_early<'a, T: 'a>(&self, x: &'a T) -> impl Iterator<Item = &'a T> where Self: Sized + 'a {
        Self::early(x)
    }

    fn extended_late<'a, T>(&self, x: &'a T) -> impl Iterator<Item = &'a T> where Self: Sized + 'a {
        self.late(x)
    }
}

impl<T> FooExt for T where T: Foo {}

fn main() {}