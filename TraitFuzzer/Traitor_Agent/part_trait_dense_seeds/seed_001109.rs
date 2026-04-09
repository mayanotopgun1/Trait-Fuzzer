#![feature(return_position_impl_trait_in_trait)]

trait Main {
    fn runner(&self) -> impl Fn();
}

impl Main for () {
    fn runner(&self) -> impl Fn() {
        || {}
    }
}

fn main() {
    let unit = ();
    unit.runner()();
}