#![allow(warnings)]

#[derive(Debug)]
pub struct Foo<T>(pub T);

use std::fmt;

impl<T> Field for T {}
impl<T> Finish for T {}
impl Dt for &mut fmt::Formatter<'_> {}

pub trait Field {
    fn field(&self, _: impl Sized) {
        panic!("got into field");
    }
}

pub trait Finish {
    fn finish(&self) -> Result<(), std::fmt::Error> {
        panic!("got into finish");
    }
}

pub trait Dt {
    fn debug_tuple(&self, _: &str) {
        panic!("got into debug_tuple");
    }
}

trait FieldExt: Field {
    fn field_twice(&self, x: impl Sized, y: impl Sized) {
        self.field(x);
        self.field(y);
    }
}

impl<T> FieldExt for T where T: Field {}

trait FinishExt: Finish {
    fn finish_twice(&self) -> Result<(), std::fmt::Error> {
        self.finish()?;
        self.finish()
    }
}

impl<T> FinishExt for T where T: Finish {}

trait DtExt: Dt {
    fn debug_tuple_twice(&self, x: &str, y: &str) {
        self.debug_tuple(x);
        self.debug_tuple(y);
    }
}

impl<T> DtExt for T where T: Dt {}

fn main() {
    let foo = Foo(());
    assert_eq!("Foo(())", format!("{:?}", foo));
}