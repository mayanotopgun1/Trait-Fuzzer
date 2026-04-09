#![allow(dead_code)]
#![feature(return_position_impl_trait_in_trait)]

pub fn main() {
    let _ = Foo::<dyn FooTrait>::new();
}

pub struct Foo<T: FooTrait + ?Sized> {
    base: FooBase,
    value: T,
}

impl<T: FooTrait + ?Sized> Foo<T> {
    pub fn new() -> Box<Foo<T>> {
        todo!()
    }
}

pub trait FooTrait {}

trait FooExt: FooTrait {
    fn foo_method(&self) {}
}

impl<T: FooTrait> FooExt for T {}

pub struct FooBase {
    cls: Bar,
}

trait FooBaseTrait {
    fn new_base() -> impl FooBaseTrait;
}

impl FooBaseTrait for FooBase {
    fn new_base() -> impl FooBaseTrait {
        FooBase { cls: Bar::default() }
    }
}

pub enum Bar {
    DefaultVariant,
}

impl Default for Bar {
    fn default() -> Self {
        Bar::DefaultVariant
    }
}