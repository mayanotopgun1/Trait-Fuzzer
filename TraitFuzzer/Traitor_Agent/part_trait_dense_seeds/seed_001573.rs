#![feature(type_alias_impl_trait)]
#![feature(impl_trait_in_assoc_type)]

trait Greet {
    type Output;
    fn greet(&self) -> Self::Output;
}

impl Greet for () {
    type Output = impl core::fmt::Debug + 'static;
    fn greet(&self) -> Self::Output {
        println!("Hello world!");
        "Hello world!"
    }
}

fn main() {
    let unit = ();
    unit.greet();
}