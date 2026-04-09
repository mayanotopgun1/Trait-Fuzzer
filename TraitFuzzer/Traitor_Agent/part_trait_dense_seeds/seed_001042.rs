#![deny(rust_2021_prelude_collisions)]

pub struct MySeq {}

trait FromIterable {
    fn from_iter(_: impl IntoIterator<Item = u32>);
}

impl FromIterable for MySeq {
    fn from_iter(_: impl IntoIterator<Item = u32>) {}
}

fn main() {
    MySeq::from_iter(Some(22));
}