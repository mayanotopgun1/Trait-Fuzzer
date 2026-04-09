#![crate_type = "lib"]

pub trait Foo<'a, T> {
    type Out;
    fn foo(&'a self) -> Self::Out;
}

trait FooExt<'a, T>: Foo<'a, T> {
    fn foo_ext(&'a self) -> Self::Out {
        self.foo()
    }
}

impl<'a, S: ?Sized, T> FooExt<'a, T> for S where S: Foo<'a, T> {}

pub trait FooPair<'a, T>: Foo<'a, T> {
    fn foo_pair(&'a self) -> (Self::Out, Self::Out)
    where
        Self::Out: Copy,
    {
        let v = self.foo();
        (v, v)
    }
}

impl<'a, S: ?Sized, T> FooPair<'a, T> for S where S: Foo<'a, T>, <S as Foo<'a, T>>::Out: Copy {}

pub fn foo<'a, T>(x: &'a dyn Foo<'a, T, Out = T>) -> T {
    x.foo_ext()
}

pub fn foo_pair<'a, T: std::marker::Copy>(x: &'a dyn FooPair<'a, T, Out = T>) -> (T, T) {
    x.foo_pair()
}