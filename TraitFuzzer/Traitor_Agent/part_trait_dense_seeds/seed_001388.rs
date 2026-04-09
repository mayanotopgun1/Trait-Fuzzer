#![feature(impl_trait_in_assoc_type)]

pub trait ArrayFactory {
    type Item;
    fn create_array() -> [Self::Item; 8];
}

impl ArrayFactory for () {
    type Item = u8;

    fn create_array() -> [u8; 8] {
        [0; 8]
    }
}

pub fn main() {
    let _foo: [u8; 8] = <() as ArrayFactory>::create_array();
}