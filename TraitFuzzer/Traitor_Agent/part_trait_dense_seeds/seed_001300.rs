#![allow(dead_code)]
#![feature(impl_trait_in_assoc_type)]

struct Inner<I, V> {
    iterator: I,
    item: V,
}

trait Iterable {
    type Item;
    fn get_iterator(self) -> Self::Item;
}

impl<I: Iterator> Iterable for Outer<I> {
    type Item = impl Iterator<Item = I::Item>;
    fn get_iterator(mut self) -> Self::Item {
        self.inner.iterator
    }
}

struct Outer<I: Iterator> {
    inner: Inner<I, I::Item>,
}

fn outer<I>(iterator: I) -> Outer<I>
where I: Iterator,
      I::Item: Default,
{
    Outer {
        inner: Inner {
            iterator: iterator,
            item: Default::default(),
        }
    }
}

fn main() {
    let iter = std::iter::once(&1).cloned();
    let outer_instance = outer(iter);
    let _iterator = outer_instance.get_iterator().collect::<Vec<_>>();
}