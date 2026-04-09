#![feature(type_alias_impl_trait)]
#![feature(impl_trait_in_assoc_type)]

pub trait FooTrait {
    type Item;
    fn foo() -> Self::Item;
}

impl FooTrait for Bar {
    type Item = Bar;
    fn foo() -> Self::Item {
        Bar
    }
}

struct Bar;

impl PartialEq<(Bar, i32)> for Bar {
    fn eq(&self, _other: &(Bar, i32)) -> bool {
        true
    }
}

fn main() {}