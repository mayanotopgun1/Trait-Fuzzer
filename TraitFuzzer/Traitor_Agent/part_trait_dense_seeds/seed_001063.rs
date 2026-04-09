#![feature(min_generic_const_args, adt_const_params)]
#![expect(incomplete_features)]
#![allow(dead_code)]

use std::marker::ConstParamTy;

#[derive(PartialEq, Eq, ConstParamTy)]
struct Container {
    values: [u32; 3],
}

trait TakesContainer<const C: Container> {
    fn execute() -> impl Iterator<Item = u32>;
}
impl<const C: Container> TakesContainer<{C}> for () {
    fn execute() -> impl Iterator<Item = u32> {
        C.values.iter().cloned()
    }
}

fn generic_caller<const N: u32, const M: u32>() {
    let _ = <() as TakesContainer<{ Container { values: [N, M, 1] } }>>::execute().collect::<Vec<_>>();
    let _ = <() as TakesContainer<{ Container { values: [1, 2, 3] } }>>::execute().collect::<Vec<_>>();
}

fn main() {}