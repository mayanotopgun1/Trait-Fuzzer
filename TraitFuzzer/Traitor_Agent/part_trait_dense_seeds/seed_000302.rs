#![feature(return_position_impl_trait_in_trait)]
#![allow(dead_code)]

trait NestedOperations {
    fn method_push_local(&mut self) -> impl core::fmt::Debug;
}

struct HasNested {
    nest: Vec<Vec<isize>>,
}

impl NestedOperations for HasNested {
    fn method_push_local(&mut self) -> impl core::fmt::Debug {
        self.nest[0].push(0);
        &self.nest[0]
    }
}

pub fn main() {}