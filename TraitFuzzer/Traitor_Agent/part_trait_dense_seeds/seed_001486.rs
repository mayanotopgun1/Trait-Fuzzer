#![feature(specialization)]

pub trait Foo {
    fn abc() -> u32;
    fn def() -> u32;
}

pub trait Marker {}

impl Marker for () {}

trait FooExt: Foo {
    fn specialized_def(&self) -> u32 {
        Self::def()
    }
}

impl<T> FooExt for T where T: Foo {}

impl<T> Foo for T {
    default fn abc() -> u32 { 16 }
    default fn def() -> u32 { 42 }
}

impl<T: Marker> Foo for T {
    fn def() -> u32 {
        Self::abc()
    }
}

fn main() {
   assert_eq!(<() as FooExt>::specialized_def(&()), 16);
   assert_eq!(<i32 as FooExt>::specialized_def(&0), 42);
}