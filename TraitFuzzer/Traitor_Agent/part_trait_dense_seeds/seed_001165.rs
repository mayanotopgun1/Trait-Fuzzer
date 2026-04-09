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

fn c1<T>(x: T) -> impl F1<T> + F2<T> {
    c1 { x }
}

trait F2<T> {
    fn f2(&self, _x: T);
}

impl<T> F2<T> for c1<T> {
    fn f2(&self, _x: T) {}
}

pub fn main() {
    let instance = c1::<isize>(3);
    instance.f1(4);
    instance.f2(4);
}