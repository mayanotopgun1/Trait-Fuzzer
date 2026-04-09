struct Foo<T>(T);

trait Extend<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I);
}

impl<T> Extend<T> for Foo<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, _: I) {
        todo!()
    }
}

impl<'a, T: 'a + Copy> Extend<&'a T> for Foo<T> {
    fn extend<I: IntoIterator<Item = &'a T>>(&mut self, iter: I) {
        <Self as Extend<T>>::extend(self, iter.into_iter().copied())
    }
}

fn main() {}