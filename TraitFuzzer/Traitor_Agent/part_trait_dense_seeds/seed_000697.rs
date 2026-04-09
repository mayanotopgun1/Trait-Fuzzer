#![allow(dead_code)]
#![allow(stable_features)]

#![feature(const_indexing, const_trait_impl)]

const trait ArrayAccess {
    type Item;
    fn get(&self, index: usize) -> Self::Item;
}

impl<const N: usize> const ArrayAccess for [i32; N] {
    type Item = i32;
    fn get(&self, index: usize) -> Self::Item {
        self[index]
    }
}

const trait ArrayExt<const N: usize>: const ArrayAccess<Item = i32> {
    fn first(&self) -> i32;
}

impl<const N: usize> const ArrayExt<N> for [i32; N] {
    fn first(&self) -> i32 {
        self.get(0)
    }
}

fn main() {
    const ARR: [i32; 6] = [42, 43, 44, 45, 46, 47];
    const IDX: usize = 3;
    const VAL: i32 = ARR.get(IDX);
    const BLUB: [i32; (ARR.first() - 41) as usize] = [5];
}