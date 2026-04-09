trait Iterable {
    type Item<'a>
    where
        Self: 'a;
    type Iter<'a>: Iterator<Item = Self::Item<'a>>
    where
        Self: 'a;

    fn iter<'a>(&'a self) -> Self::Iter<'a>;
}

trait IterableExt: Iterable {
    fn first_item<'a>(&'a self) -> Option<Self::Item<'a>> {
        self.iter().next()
    }
}

impl<T> Iterable for [T] {
    type Item<'a> = <std::slice::Iter<'a, T> as Iterator>::Item where T: 'a;
    type Iter<'a> = std::slice::Iter<'a, T> where T: 'a;

    fn iter<'a>(&'a self) -> Self::Iter<'a> {
        self.iter()
    }
}

impl<T: Iterable + ?Sized> IterableExt for T {}

fn main() {
    let v = vec![1, 2, 3];

    assert_eq!(Some(&1), v.first_item());
}