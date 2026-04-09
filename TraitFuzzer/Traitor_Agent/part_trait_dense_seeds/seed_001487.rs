#![allow(dead_code)]

fn produce_static<T>() -> &'static T { panic!(); }

trait StaticProducer {
    fn produce_static(&self) -> &'static usize;
}

impl<T> StaticProducer for T {
    fn produce_static(&self) -> &'static usize {
        produce_static()
    }
}

fn foo<T: StaticProducer>(_x: &T) -> &usize {
    _x.produce_static()
}

pub fn main() {}