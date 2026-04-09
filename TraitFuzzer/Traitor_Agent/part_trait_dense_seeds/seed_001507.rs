#![allow(unused_variables)]

struct Point {
    x: isize,
    y: isize,
}

trait Origin {
    fn origin() -> Self;
}

impl Origin for Point {
    fn origin() -> Self {
        Point { x: 0, y: 0 }
    }
}

fn main() {
    let origin = Point::origin();
    let f: Point = Point { x: (panic!("beep boop")), ..origin };
}