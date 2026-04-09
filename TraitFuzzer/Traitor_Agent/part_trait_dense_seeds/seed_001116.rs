#![feature(specialization)]

enum E { A, }

const C: [u32; 1] = [1];

trait Accessor {
    fn access(&self, index: usize) -> u32;
}

default impl<T> Accessor for T {
    default fn access(&self, _index: usize) -> u32 {
        panic!("Not implemented")
    }
}

impl Accessor for [u32; 1] {
    fn access(&self, index: usize) -> u32 {
        self[index]
    }
}

fn main() {
    let a = C.access(E::A as usize);
}