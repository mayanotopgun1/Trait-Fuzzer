#![feature(associated_type_defaults)]

pub trait Tr {
    type Assoc = u8;
    type Assoc2 = Self::Assoc;
    const C: u8 = 11;
    fn foo(&self) -> impl core::fmt::Debug where Self: core::fmt::Debug { self }
}

trait TrExt: Tr {
    fn foo_twice(&self) -> impl core::fmt::Debug where Self: core::fmt::Debug {
        let first = self.foo();
        let second = self.foo();
        (first, second)
    }
}

impl<T: Tr> TrExt for T {}

impl Tr for () {
    type Assoc = ();
}

fn main() {
    let _: <() as Tr>::Assoc = ();
    let _: <() as Tr>::Assoc2 = ();
}