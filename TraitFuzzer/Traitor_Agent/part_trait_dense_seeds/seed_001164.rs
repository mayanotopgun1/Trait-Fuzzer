#![allow(dead_code)]
#![allow(non_camel_case_types)]

struct c1<T> {
    x: T,
}

trait F1<T> {
    fn f1(&self, _x: T);
}

impl<T> F1<T> for c1<T> {
    fn f1(&self, _x: T) {}
}

fn c1<T>(x: T) -> c1<T> {
    c1 { x }
}

trait F2<T> {
    fn f2(&self, _x: T);
}

impl<T> F2<T> for c1<T> {
    fn f2(&self, _x: T) {}
}

pub fn main() {
    c1::<isize>(3).f1(4);
    c1::<isize>(3).f2(4);
}