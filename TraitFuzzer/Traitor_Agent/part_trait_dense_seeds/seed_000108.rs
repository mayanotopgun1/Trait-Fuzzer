#![allow(dead_code)]

trait VecExt {
    fn new_vec(&self) -> Vec<isize>;
}

impl VecExt for () {
    fn new_vec(&self) -> Vec<isize> {
        Vec::new()
    }
}

fn f() -> Vec<isize> { ().new_vec() }

pub fn main() {}