#![allow(bare_trait_objects)]

pub trait SomeTrait {}

pub trait ProcessTrait {
    fn process(self);
}

impl<T: SomeTrait> ProcessTrait for T {
    fn process(self) {}
}

pub fn function(_x: Box<SomeTrait>) {}

fn main() {}