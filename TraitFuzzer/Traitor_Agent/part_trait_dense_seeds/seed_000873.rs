#![feature(type_alias_impl_trait)]

trait PanicTrait<'a, T: ?Sized> {
    type Output;
    fn panic(&self) -> Self::Output;
}

impl<'a, T: ?Sized + 'a> PanicTrait<'a, T> for () {
    type Output = Option<&'a T>;
    fn panic(&self) -> Self::Output {
        extern "C" fn _panic<'b, U: ?Sized>() -> Option<&'b U> {
            panic!()
        }
        _panic()
    }
}

fn main() {}