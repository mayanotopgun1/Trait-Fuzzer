#![allow(stable_features)]
#![feature(const_indexing)]

trait IndexAccess<T> {
    fn access(&self, index: usize) -> T;
}

impl IndexAccess<usize> for [usize; 5] {
    fn access(&self, index: usize) -> usize {
        self[index]
    }
}

const ARR: [usize; 5] = [5, 4, 3, 2, 1];

fn main() {
    assert_eq!(3, ARR.access(ARR.access(3)));
}