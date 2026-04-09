#![feature(type_alias_impl_trait)]

pub type Opaque<T> = impl Sized;
#[define_opaque(Opaque)]
fn defining<T>() -> Opaque<T> {}

struct Ss<'a, T>(&'a Opaque<T>);

trait TestTrait<'a, T> {
    fn test(self);
}

impl<'a, T> TestTrait<'a, T> for Ss<'a, T> {
    fn test(self) {

        None::<&'a Opaque<T>>;
    }
}

fn main() {}