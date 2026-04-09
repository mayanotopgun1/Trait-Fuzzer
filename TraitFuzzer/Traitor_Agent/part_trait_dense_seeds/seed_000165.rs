#![crate_type = "lib"]

pub trait Foo<'a, T> {
    fn foo(&'a self) -> T;
}

// New trait to increase participation
trait FooExt<'a, T>: Foo<'a, T> where T: Copy {
    fn foo_twice(&'a self) -> (T, T) {
        let v = self.foo();
        (v, v)
    }
}

// Implement the new trait for any type that implements Foo
impl<'a, T, F> FooExt<'a, T> for F where F: Foo<'a, T>, T: Copy {}

pub fn foo<'a, T>(x: &'a dyn Foo<'a, T>) -> T {
    let x: &'a dyn Foo<T> = x;

    x.foo()
}