use std::vec;

pub trait BitIter {
    type Iter: Iterator<Item=bool>;
    fn bit_iter(self) -> <Self as BitIter>::Iter;
}

impl BitIter for Vec<bool> {
    type Iter = vec::IntoIter<bool>;
    fn bit_iter(self) -> <Self as BitIter>::Iter {
        self.into_iter()
    }
}

pub trait CountableBits: BitIter + Sized {
    fn count_bits(self) -> usize {
        let mut sum = 0;
        for i in self.bit_iter() {
            if i {
                sum += 1;
            }
        }
        sum
    }
}

impl<T> CountableBits for T where T: BitIter {}

fn main() {
    let v = vec![true, false, true];
    let c = v.count_bits();
    assert_eq!(c, 2);
}