#![allow(unused)]

struct Point {
    x: i32,
    y: i32,
}

trait Access {
    fn get_x(&self) -> i32;
}

impl Access for Point {
    fn get_x(&self) -> i32 {
        self.x
    }
}

struct Wrapper {
    p: Point,
}

fn main() {
    let mut w = Wrapper { p: Point { x: 10, y: 10 } };

    let c = || {
        println!("{}", w.p.get_x());
    };

    let px = &w.p.get_x();
    c();

    println!("{}", px);
}