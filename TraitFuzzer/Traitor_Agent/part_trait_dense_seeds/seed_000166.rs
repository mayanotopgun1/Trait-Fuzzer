#![crate_type = "lib"]
#![feature(specialization)]

pub trait Foo<'a, T> {
    fn foo(&'a self) -> T;
}

trait FooExt<'a, T>: Foo<'a, T> where T: Copy {
    fn foo_twice(&'a self) -> (T, T);
}

impl<'a, T, F> FooExt<'a, T> for F
where
    F: Foo<'a, T>,
    T: Copy,
{
    default fn foo_twice(&'a self) -> (T, T) {
        let v = self.foo();
        (v, v)
    }
}

pub fn foo<'a, T>(x: &'a dyn Foo<'a, T>) -> T {
    x.foo()
}