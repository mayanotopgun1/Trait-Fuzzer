#![allow(non_camel_case_types)]

trait UnionTrait {
    fn new() -> Self;
}

struct union;

impl UnionTrait for union {
    fn new() -> Self {
        union { }
    }
}

fn main() {
    let _u = union::new();
}