#![feature(impl_trait_in_assoc_type)]

macro_rules! m {
    ($p: path) => {
        let _ = $p(0);
        let _: $p;
    }
}

trait AsFoo<T> {
    fn as_foo(self) -> Foo<T>;
}

impl<T: Default> AsFoo<T> for Option<Foo<T>> {
    fn as_foo(self) -> Foo<T> {
        self.unwrap_or_else(|| Foo { _a: T::default() })
    }
}

struct Foo<T> {
    _a: T,
}

impl<T> From<S<T>> for Foo<T> {
    fn from(s: S<T>) -> Self {
        Foo { _a: s.0 }
    }
}

struct S<T>(T);

fn f() -> Foo<i32> {
    let _: Foo<i32> = Some(Foo { _a: 42 }).as_foo();
    let g: Foo<i32> = Foo { _a: 42 };

    m!(S::<u8>);
    g
}

fn main() {}