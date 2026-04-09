trait Iterate<'a> {
    type Item;
    fn iter(&'a self) -> std::slice::Iter<'a, Self::Item>;
}

impl<'a, T> Iterate<'a> for Vec<T> {
    type Item = T;
    fn iter(&'a self) -> std::slice::Iter<'a, Self::Item> {
        self.iter()
    }
}

trait IterateMut<'a> {
    type Item;
    fn iter_mut(&'a mut self) -> std::slice::IterMut<'a, Self::Item>;
}

impl<'a, T> IterateMut<'a> for Vec<T> {
    type Item = T;
    fn iter_mut(&'a mut self) -> std::slice::IterMut<'a, Self::Item> {
        self.iter_mut()
    }
}

trait IntoIteratorExt: IntoIterator {
    fn into_iter_ext(self) -> <Self as IntoIterator>::IntoIter;
}

impl<T> IntoIteratorExt for Vec<T> {
    fn into_iter_ext(self) -> <Self as IntoIterator>::IntoIter {
        self.into_iter()
    }
}

fn main() {
    let mut v = vec![1];

    for x in v.iter() {
        assert_eq!(x, &1);
    }

    for x in v.iter_mut() {
        assert_eq!(x, &mut 1);
    }

    for x in v.into_iter_ext() {
        assert_eq!(x, 1);
    }
}