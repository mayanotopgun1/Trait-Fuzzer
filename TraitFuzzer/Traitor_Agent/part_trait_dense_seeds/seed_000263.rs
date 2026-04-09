#![feature(adt_const_params, lazy_type_alias)]

pub type Matrix = [usize; 1];
const EMPTY_MATRIX: Matrix = [0; 1];

trait WalkTrait<const REMAINING: Matrix> {
    fn new() -> Self;
}

impl WalkTrait<EMPTY_MATRIX> for Walk<EMPTY_MATRIX> {
    fn new() -> Self {
        Self {}
    }
}

pub struct Walk<const REMAINING: Matrix> {}

fn main() {
    let _ = <Walk::<EMPTY_MATRIX>>::new();
}