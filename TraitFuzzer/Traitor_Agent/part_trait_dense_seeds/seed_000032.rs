#![feature(impl_trait_in_assoc_type)]

trait Bar {}
struct Dummy;
impl Bar for Dummy {}

trait Foo {
    type Assoc: Bar;
    fn foo(&self) -> Self::Assoc;
}

impl Foo for i32 {
    type Assoc = impl Bar;
    fn foo(&self) -> Self::Assoc {
        Dummy
    }
}

fn main() {}