#![crate_type = "lib"]

pub trait First {
    const CONST: bool;
}
pub trait Second {}

pub trait FirstExt<'a>: First {}
impl<'a, T> FirstExt<'a> for T where &'a Self: First, T: First + 'a {}

impl<'a> First for dyn Second where &'a Self: First {
    const CONST: bool = <&Self>::CONST;
}

pub trait Foo {
    const CONST: bool;
}
pub trait FooExt<'a>: Foo {}
impl<'a, T> FooExt<'a> for T where &'a Self: Foo, T: Foo + 'a {}

impl<'a> Foo for () where &'a Self: Foo {
    const CONST: bool = <&Self>::CONST;
}