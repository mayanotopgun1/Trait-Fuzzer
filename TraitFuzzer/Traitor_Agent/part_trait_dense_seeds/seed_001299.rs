#![allow(dead_code)]

struct Inner<I, V> {
    iterator: I,
    item: V,
}

trait Iterable {
    type Item;
    fn get_iterator(self) -> Self::Item;
}

impl<I: Iterator> Iterable for Outer<I> {
    type Item = I;
    fn get_iterator(self) -> I {
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
    let _iterator = outer_instance.get_iterator();
}