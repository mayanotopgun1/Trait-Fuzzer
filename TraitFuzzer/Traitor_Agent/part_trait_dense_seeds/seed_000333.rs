#![allow(unused_variables)]

use std::collections::HashMap;

trait CopyTrait<T>: Copy {
    fn copy(&self) -> T;
}

impl<T: Copy> CopyTrait<T> for &T {
    fn copy(&self) -> T {
        **self
    }
}

fn main() {
    let arr = [(1, 1), (2, 2), (3, 3)];

    let v1: Vec<&_> = arr.iter().collect();
    let v2: Vec<_> = arr.iter().map(|x| x.copy()).collect();

    let m1: HashMap<_, _> = arr.iter().map(|x| x.copy()).collect();
    let m2: HashMap<isize, _> = arr.iter().map(|x| x.copy()).collect();
    let m3: HashMap<_, usize> = arr.iter().map(|x| x.copy()).collect();
}