#![allow(unused)]
#![feature(impl_trait_in_assoc_type)]

struct Point {
    x: i32,
    y: i32,
}

trait Access {
    type X;
    fn get_x(&self) -> Self::X;
}

impl Access for Point {
    type X = impl core::fmt::Debug + Copy;
    fn get_x(&self) -> Self::X {
        self.x
    }
}

struct Wrapper {
    p: Point,
}

fn main() {
    let mut w = Wrapper { p: Point { x: 10, y: 10 } };

    let c = || {
        println!("{:?}", w.p.get_x());
    };

    let px = &w.p.get_x();
    c();

    println!("{:?}", px);
}