#![feature(return_position_impl_trait_in_trait)]

trait Each<T> {
    fn each<'a>(&'a self) -> impl Iterator<Item = &'a T> where T: 'a;
}

impl<T> Each<T> for [T] {
    fn each<'a>(&'a self) -> impl Iterator<Item = &'a T> where T: 'a {
        self.iter()
    }
}

fn main() {
    let mut sum = 0_usize;
    let elems = [1_usize, 2, 3, 4, 5];
    for val in elems.each() {
        sum += *val;
    }
    assert_eq!(sum, 15);
}