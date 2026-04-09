pub trait Foo<T> {
    fn foo(self) -> T;
}

trait Bar<T>: Foo<T> {
    fn bar(&self) -> Option<T> {
        None
    }
}

impl<S, T> Bar<T> for S where S: Foo<T> {}

impl<'a, T> Foo<T> for &'a str where &'a str: Into<T> {
    fn foo(self) -> T {
        panic!();
    }
}

fn main() {}