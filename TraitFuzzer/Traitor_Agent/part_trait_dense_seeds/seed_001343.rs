#![allow(dead_code)]

trait DataHolder {
    type Data;
    fn get_data(&self) -> &Self::Data;
}

impl<T> DataHolder for Foo<T> {
    type Data = T;
    fn get_data(&self) -> &Self::Data {
        &self.data
    }
}

pub struct Foo<T> {
    data: T,
}

fn foo<T, F: DataHolder<Data=T>>(_: &F) {
}

pub fn main() {
}