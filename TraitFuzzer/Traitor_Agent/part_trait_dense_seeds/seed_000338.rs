#![allow(non_camel_case_types)]
#![feature(impl_trait_in_fn_trait_return)]

trait UnionTrait {
    fn new() -> Self;
}

struct union;

impl UnionTrait for union {
    fn new() -> union {
        union { }
    }
}

fn main() {
    let _u = union::new();
}