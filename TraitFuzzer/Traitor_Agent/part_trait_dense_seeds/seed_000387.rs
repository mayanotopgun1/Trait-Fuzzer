#![feature(generic_associated_types)]

trait Foo<'a> {
    type Output;
    fn bar(&self) -> Self::Output;
}

impl<'a> Foo<'a> for Baz {
    type Output = &'static str;
    fn bar(&self) -> Self::Output {
        "foo"
    }
}

struct Baz {}

trait QuxTrait<'a> {
    type Output;
    fn bar(&self) -> Self::Output;
}

struct Qux {}

impl<'a> QuxTrait<'a> for Qux {
    type Output = &'static str;
    fn bar(&self) -> Self::Output {
        "qux"
    }
}

trait QuuxTrait {
    fn quux(&self);
}

impl QuuxTrait for () {
    fn quux(&self) {
        println!("quux");
    }
}

pub fn main() {
    let baz = Baz {};
    println!("{}", baz.bar());
    let qux = Qux {};
    println!("{}", qux.bar());
    (()).quux();
}