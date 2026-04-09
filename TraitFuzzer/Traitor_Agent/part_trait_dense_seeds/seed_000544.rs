#![feature(return_position_impl_trait_in_trait)]

pub trait MainTrait {
    fn runner(&self) -> impl Fn();
}

impl MainTrait for () {
    fn runner(&self) -> impl Fn() {
        || {}
    }
}

pub fn main() {
    let _: () = ();
    ().runner()();
}