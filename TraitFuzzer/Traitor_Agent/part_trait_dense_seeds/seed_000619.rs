#![feature(return_position_impl_trait_in_trait)]

trait Super {}

trait Sub<T>: Super {}

trait Overlap<T> {}
impl<T, U: Sub<T>> Overlap<T> for U {}
impl<T> Overlap<T> for () {}

trait EmptyCheck {
    fn is_empty(&self) -> impl Fn() -> bool;
}

impl<U: Overlap<()>> EmptyCheck for U {
    fn is_empty(&self) -> impl Fn() -> bool {
        || true
    }
}

fn main() {}