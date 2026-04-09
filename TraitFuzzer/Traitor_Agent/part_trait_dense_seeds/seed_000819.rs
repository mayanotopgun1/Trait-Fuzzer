#![feature(generic_associated_types)]

use std::vec;

pub trait BitIter {
    type Iter<'a>: Iterator<Item=bool> where Self: 'a;
    fn bit_iter(&self) -> <Self as BitIter>::Iter<'_>;
}

trait BitCountExt: BitIter + Clone {
    fn count_bits_ext(&self) -> usize {
        let mut sum = 0;
        for i in self.bit_iter() {
            if i {
                sum += 1;
            }
        }
        sum
    }
}

impl<T> BitCountExt for T where T: BitIter + Clone {}

trait BitCount: BitIter + Clone {
    fn count_bits(&self) -> usize;
}

impl<T> BitCount for T where T: BitCountExt + Clone {
    fn count_bits(&self) -> usize {
        self.count_bits_ext()
    }
}

impl BitIter for Vec<bool> {
    type Iter<'a> = vec::IntoIter<bool>;
    fn bit_iter(&self) -> <Self as BitIter>::Iter<'_> {
        self.clone().into_iter()
    }
}

fn main() {
    let v = vec![true, false, true];
    let c = v.count_bits();
    assert_eq!(c, 2);
}