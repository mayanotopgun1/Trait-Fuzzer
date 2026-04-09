#![feature(generic_associated_types)]
#![allow(unused_variables)]

struct Point {
    x: isize,
    y: isize,
}

trait Origin<'a> {
    type Out;
    fn origin() -> Self::Out;
}

impl<'a> Origin<'a> for Point {
    type Out = Self;
    fn origin() -> Self::Out {
        Point { x: 0, y: 0 }
    }
}

fn main() {
    let origin = <Point as Origin>::origin();
    let f: Point = Point { x: (panic!("beep boop")), ..origin };
}