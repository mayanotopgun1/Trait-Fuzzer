#![feature(never_type)]

trait Pushable {
    fn push(&mut self, item: (usize, !));
}

impl<T> Pushable for Vec<T> {
    fn push(&mut self, item: (usize, !)) {
        std::panic!("This should never happen")
    }
}

fn main() {
    let mut vec = vec![];
    vec.push((vec.len(), panic!()));
}