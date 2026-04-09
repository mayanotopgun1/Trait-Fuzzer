#![feature(return_position_impl_trait_in_trait)]

struct W<T, U>(T, U);

trait Trait<T> {}

impl<'a> Trait<W<&'a str, &'a str>> for () {}
impl<'a> Trait<W<&'a str, String>> for () {}

trait NotString {}
impl NotString for &str {}
impl NotString for u32 {}


trait Overlap<U> {
    fn check(&self) -> impl std::fmt::Debug;
}

impl<T: for<'a> Trait<W<&'a str, U>>, U> Overlap<U> for T {
    fn check(&self) -> impl std::fmt::Debug {
        true
    }
}

impl<U: NotString> Overlap<U> for () {
    fn check(&self) -> impl std::fmt::Debug {
        false
    }
}

fn main() {}