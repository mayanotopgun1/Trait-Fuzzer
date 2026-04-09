#![allow(dead_code)]
#![feature(impl_trait_in_assoc_type)]

trait ArrLike {
    type Item;
    fn get(&self, index: usize) -> Self::Item;
}

trait ArrDebug: ArrLike where Self::Item: core::fmt::Debug {
    fn debug_get(&self, index: usize) -> String {
        format!("{:?}", self.get(index))
    }
}

impl<T> ArrDebug for T where T: ArrLike, T::Item: core::fmt::Debug {}

#[derive(Copy, Clone)]
struct Array {
    arr: [[u8; 256]; 4]
}

impl ArrLike for Array {
    type Item = [u8; 256];
    fn get(&self, index: usize) -> Self::Item {
        self.arr[index]
    }
}

pub fn main() {}