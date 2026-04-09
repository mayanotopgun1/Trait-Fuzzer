#![feature(impl_trait_in_assoc_type)]

trait A {
    type B<'a>: core::fmt::Debug where Self: 'a;

    fn make_b<'a>(&'a self) -> Self::B<'a>;
}

#[derive(Debug)]
struct S {}
impl A for S {
    type B<'a> = &'a S;
    fn make_b<'a>(&'a self) -> Self::B<'a> {
        self
    }
}

enum E<'a, T: 'a + A> {
    S(T::B<'a>),
}

fn main() {}