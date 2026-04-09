#![crate_type="lib"]
#![feature(impl_trait_in_assoc_type)]

pub trait Foo {
    const BAR: usize;
}

pub struct FooNoDefault;

impl Foo for FooNoDefault {
    const BAR: usize = 0;
}


pub trait FooDefault {
    const BAR: usize = 1;
}

pub struct FooOverwriteDefault;

impl FooDefault for FooOverwriteDefault {
    const BAR: usize = 2;
}

pub struct FooUseDefault;

impl FooDefault for FooUseDefault {}


pub struct InherentBar;

trait BarTrait {
    const BAR: usize;
}

impl BarTrait for InherentBar {
    const BAR: usize = 3;
}