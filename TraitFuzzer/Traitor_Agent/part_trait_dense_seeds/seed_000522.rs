#![feature(impl_trait_in_assoc_type)]

mod baz {
    struct Foo;

    pub trait Trait {
        type Assoc;
    }

    impl Trait for Foo {
        type Assoc = ();
    }

    pub trait BarTrait<'a, T: Trait> {
        fn new(source: &'a T::Assoc) -> Self;
    }

    impl<'a, T: Trait> BarTrait<'a, T> for Bar<'a, T> {
        fn new(source: &'a T::Assoc) -> Self {
            Bar { source }
        }
    }

    pub struct Bar<'a, T: Trait> {
        source: &'a T::Assoc,
    }

    pub trait BazTrait<'a> {
        fn new(mode: Bar<'a, Foo>) -> Self;
    }

    impl<'a> BazTrait<'a> for Baz<'a> {
        fn new(mode: Bar<'a, Foo>) -> Self {
            Baz { mode }
        }
    }

    pub struct Baz<'a> {
        mode: Bar<'a, Foo>,
    }
}

pub struct Struct<'a> {
    lexer: baz::Baz<'a>,
}

fn main() {}