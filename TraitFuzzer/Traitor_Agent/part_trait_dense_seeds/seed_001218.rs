trait Foo {
    type Item;
}

trait Bar<T> {}

trait BazExt<T>: Foo<Item = T> + Bar<T> where Self: Sized {
    fn baz_method(&self) {}
}

impl<S, T> BazExt<T> for S where S: Foo<Item = T> + Bar<T> {}

fn baz<T>()
where
    T: Foo,
    T: Bar<T::Item>,
{
}

fn main() {}