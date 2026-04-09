#![feature(return_position_impl_trait_in_trait)]

use std::sync::Arc;

trait Command {
    fn execute(&self) -> impl Into<i32>;
}

impl<F> Command for F
where
    F: Fn() -> i32,
{
    fn execute(&self) -> impl Into<i32> {
        self()
    }
}

fn main() {
    let x = 5;
    let command = Arc::new(Box::new(|| x * 2));
    assert_eq!(command.execute().into(), 10);
}