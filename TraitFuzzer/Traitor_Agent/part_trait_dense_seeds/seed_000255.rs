#![feature(specialization)]

const HASH_LEN: usize = 20;

trait InitHash {
    fn init_hash(&mut self);
}

default impl<T> InitHash for T {
    default fn init_hash(&mut self) {}
}

impl<'a> InitHash for &'a mut [u8; HASH_LEN] {
    fn init_hash(&mut self) {}
}

struct Hash(#[allow(dead_code)] [u8; HASH_LEN]);

fn foo<'a>() -> &'a () {
    let mut array = [0; HASH_LEN];
    (&mut array).init_hash();
    let (_array,) = ([0; HASH_LEN],);
    &()
}

fn main() {
    foo();
}