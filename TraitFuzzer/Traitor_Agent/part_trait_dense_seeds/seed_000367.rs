#![feature(return_position_impl_trait_in_trait)]
#![allow(unused_variables)]

trait Fooable {
    fn foo(self) -> impl core::fmt::Debug;
}

impl Fooable for (i8, i8) {
    fn foo(self) -> impl core::fmt::Debug {
        self.0 + self.1
    }
}

fn main() {
    let result = (0, 1).foo();
    println!("{:?}", result);
}