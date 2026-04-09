#![feature(min_generic_const_args, adt_const_params)]
#![expect(incomplete_features)]
#![allow(dead_code)]

use std::marker::ConstParamTy;

#[derive(PartialEq, Eq, ConstParamTy)]
struct Container {
    values: [u32; 3],
}

trait TakesContainer<const C: Container> {
    fn execute();
}
impl<const C: Container> TakesContainer<{C}> for () {
    fn execute() {}
}

fn generic_caller<const N: u32, const M: u32>() {
    <() as TakesContainer<{ Container { values: [N, M, 1] } }>>::execute();
    <() as TakesContainer<{ Container { values: [1, 2, 3] } }>>::execute();
}

fn main() {}