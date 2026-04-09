#![feature(return_position_impl_trait_in_trait)]

trait Bar {}
struct Dummy;
impl Bar for Dummy {}

trait Foo {
    fn foo(&self) -> impl Bar;
}

impl Foo for i32 {
    fn foo(&self) -> impl Bar {
        Dummy
    }
}

fn main() {}