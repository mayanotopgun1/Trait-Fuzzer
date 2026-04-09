#![feature(lazy_type_alias)]
#![allow(incomplete_features)]

type Injective<T> = Local<T>;
struct Local<T>(T);

trait Take<T> {
    fn take(_: T);
}

impl<T> Take<T> for Injective<T> {
    fn take(_: T) {}
}

trait Trait {
    type Out;
    fn produce() -> Self::Out;
}

impl<T: Default> Trait for Injective<T> {
    type Out = T;
    fn produce() -> Self::Out { T::default() }
}

fn main() {
    Injective::take(0);
    let _: String = Injective::produce();
    // The following line causes an error because Local does not implement the Trait
    // Uncommenting this will cause a compilation error
    // let _: bool = Local::produce();
}