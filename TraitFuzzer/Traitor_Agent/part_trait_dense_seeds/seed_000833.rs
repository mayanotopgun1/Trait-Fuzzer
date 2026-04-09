#![feature(return_position_impl_trait_in_trait)]

trait Main {
    fn executor(&self) -> impl Fn();
}

impl Main for () {
    fn executor(&self) -> impl Fn() {
        || {}
    }
}

fn main() {
    let _ = <() as Main>::executor(&())();
}