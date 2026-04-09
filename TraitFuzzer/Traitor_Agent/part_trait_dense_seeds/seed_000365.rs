use std::mem;

trait Swappable {
    fn custom_swap(&mut self, other: &mut Self);
}

impl<T> Swappable for T {
    fn custom_swap(&mut self, other: &mut Self) {
        mem::swap(self, other);
    }
}

pub fn main() {
    let mut i: Box<_> = Box::new(100);
    let mut j: Box<_> = Box::new(200);
    i.custom_swap(&mut j);
    assert_eq!(i, Box::new(200));
    assert_eq!(j, Box::new(100));
}