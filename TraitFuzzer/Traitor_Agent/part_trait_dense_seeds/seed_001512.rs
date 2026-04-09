#![feature(specialization)]

macro_rules! foo {
    ( $(banana $a:ident)* $(orange $b:tt)* ) => { };
}

trait FooTrait {
    fn foo(&self);
}

default impl<T> FooTrait for T {
    default fn foo(&self) {}
}

impl FooTrait for () {
    fn foo(&self) {}
}

fn main() {
    let unit = ();
    unit.foo();
    foo!( banana id1 banana id2
          orange hi  orange (hello world) );
}