#![no_std]
#![feature(impl_trait_in_assoc_type)]

extern crate std;

trait UnwrapOr<T> {
    type Out;
    fn unwrap(self) -> Self::Out;
}

impl<T: core::fmt::Debug + 'static> UnwrapOr<T> for Option<T> {
    type Out = T;
    fn unwrap(self) -> Self::Out {
        self.unwrap()
    }
}

fn main() {
    let a: Option<&str> = Some("foo");
    let _ = a.unwrap();
}