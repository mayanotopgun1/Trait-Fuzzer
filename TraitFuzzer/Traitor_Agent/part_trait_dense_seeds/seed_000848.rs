#![feature(return_position_impl_trait_in_trait)]

trait Tr {
    fn foo(&self);
}

trait BarExt: Tr {
    fn bar(&self) -> impl core::fmt::Debug {
        (|| { self.foo() })()
    }
}

impl<T> BarExt for T where T: Tr {}

impl Tr for () {
    fn foo(&self) {}
}

fn main() {
    ().bar();
}