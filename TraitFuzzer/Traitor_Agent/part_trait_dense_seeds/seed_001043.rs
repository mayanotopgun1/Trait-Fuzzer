#![deny(rust_2021_prelude_collisions)]
#![feature(impl_trait_in_assoc_type)]

pub struct MySeq {}

trait FromIterable {
    fn from_iter(_: impl IntoIterator<Item = u32>) -> Self;
}

impl FromIterable for MySeq {
    fn from_iter(_: impl IntoIterator<Item = u32>) -> Self {
        MySeq {}
    }
}

fn main() {
    let _ = MySeq::from_iter(Some(22));
}