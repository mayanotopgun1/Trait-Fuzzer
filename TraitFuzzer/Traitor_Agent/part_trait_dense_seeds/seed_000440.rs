#![feature(return_position_impl_trait_in_trait)]

#[deprecated]
trait MainTrait {
    fn run(&self) -> impl core::fmt::Debug;
}

impl MainTrait for () {
    fn run(&self) -> impl core::fmt::Debug { () }
}

fn main() {
    let _ = (()).run();
}