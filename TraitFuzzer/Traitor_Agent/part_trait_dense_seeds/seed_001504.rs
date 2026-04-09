pub trait IntoFoo {
    type Item;
    type IntoIter: Foo<Item = Self::Item> + Clone;

    fn into_iter(self) -> Self::IntoIter;
}

pub trait Foo {
    type Item;

    fn next(&self) -> Option<Self::Item>;
}

trait NextWrapper: Foo + Clone {
    fn try_next(&self) -> Option<Self::Item> where Self::Item: Copy { self.clone().next() }
}

impl<T: Foo<Item = U> + Clone, U> NextWrapper for T {}

pub trait IntoFooRef<'a>: 'a {
    type Item;
    type Iter: Foo<Item = Self::Item> + Clone;

    fn iter(&'a self) -> Self::Iter;
}

impl<'a, I, E> IntoFooRef<'a> for I
where
    I: IntoFoo<Item = E> + 'a + Clone,
{
    type Item = E;
    type Iter = <I as IntoFoo>::IntoIter;

    fn iter(&'a self) -> Self::Iter {
        self.clone().into_iter()
    }
}

pub fn foo<'a, Iter1, Elem1>(a: &'a Iter1)
where
    Iter1: 'a + IntoFooRef<'a, Item = Elem1>,
    Elem1: Copy,
{
    a.iter().try_next();
}

fn main() {}