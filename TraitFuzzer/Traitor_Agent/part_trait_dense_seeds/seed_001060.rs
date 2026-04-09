#![crate_type = "lib"]

trait ArrayTrait {
    const LEN: usize;
}

impl<T, const N: usize> ArrayTrait for [T; N] {
    const LEN: usize = N;
}

pub fn foo() {
    let s: [u8; 10];
    s = [0; <[u8; 10]>::LEN];
}