#![feature(return_position_impl_trait_in_trait)]

struct A<T, U: ?Sized + 'static>(#[allow(dead_code)] T, B<T, U>);
struct B<T, U: ?Sized>(#[allow(dead_code)] T, U);

trait Access {
    type First;
    type Second: ?Sized;

    fn inner(&self) -> &B<Self::First, Self::Second>;
}

impl<T, U: ?Sized + 'static> Access for A<T, U> {
    type First = T;
    type Second = U;

    fn inner(&self) -> &B<Self::First, Self::Second> {
        &self.1
    }
}

fn main() {
    let x: A<[u32; 1], [u32; 1]> = A([0; 1], B([0; 1], [0; 1]));
    let y: &A<[u32; 1], [u32]> = &x;
    assert_eq!(y.inner().1.len(), 1);
}