#![crate_type = "lib"]
#![feature(generic_associated_types)]

pub trait First {
    type Assoc<'a> where Self: 'a;
    const CONST: bool;
}

pub trait Second {}

impl<'a, T: ?Sized + First> First for &'a T {
    type Assoc<'b> where Self: 'b = <T as First>::Assoc<'b>;
    const CONST: bool = T::CONST;
}

pub trait Foo {
    type Assoc<'a> where Self: 'a;
    const CONST: bool;
}

pub trait FooExt: Foo {
    fn foo_twice() -> (bool, bool) {
        let v = Self::CONST;
        (v, v)
    }
}

impl<T: ?Sized + Foo> FooExt for T {}

impl <'a> Foo for () where &'a (): Foo {
    type Assoc<'b> = ();
    const CONST: bool = <&()>::CONST;
}