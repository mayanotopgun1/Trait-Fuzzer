#![feature(return_position_impl_trait_in_trait)]
#![allow(unused_variables)]

trait ClosureExecutor {
    fn execute(self) -> impl core::fmt::Debug;
}

impl ClosureExecutor for Box<dyn FnOnce()> {
    fn execute(self) -> impl core::fmt::Debug {
        self();
        "Executed"
    }
}

pub fn main() {
    let closure = || {
        let i = 10;
    };

    let boxed_closure: Box<dyn FnOnce()> = Box::new(closure);
    let result = boxed_closure.execute();
    println!("{:?}", result);
}