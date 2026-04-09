#![feature(type_alias_impl_trait)]
#![crate_type = "lib"]

trait ArrayTrait {
    type Out;
    const LEN: usize;
}

impl<T, const N: usize> ArrayTrait for [T; N] {
    type Out = [T; N];
    const LEN: usize = N;
}

pub fn foo() -> <[u8; 10] as ArrayTrait>::Out {
    let s: [u8; 10];
    s = [0; <[u8; 10]>::LEN];
    s
}