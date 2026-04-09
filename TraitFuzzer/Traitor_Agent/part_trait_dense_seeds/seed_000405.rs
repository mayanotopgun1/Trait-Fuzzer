#![allow(dead_code)]
#[derive(Debug)]
struct Pair<T, U> {
    a: T,
    b: U,
}

impl<T, U> Pair<T, U> {
    fn new(a: T, b: U) -> Self {
        Pair { a, b }
    }
}

struct Triple {
    x: isize,
    y: isize,
    z: isize,
}

trait IntoPair<T, U> {
    fn into_pair(self, y: U) -> Pair<T, U>;
}

impl<T, U> IntoPair<T, U> for T {
    fn into_pair(self, y: U) -> Pair<T, U> {
        Pair { a: self, b: y }
    }
}

pub fn main() {
    println!("{}", Triple { x: 3, y: 4, z: 5 }.into_pair(4).a.x);
    println!("{}", 5.into_pair(6).a);
}