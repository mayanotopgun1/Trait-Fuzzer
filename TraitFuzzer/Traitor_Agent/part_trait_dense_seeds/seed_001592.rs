#![feature(specialization)]

trait Adder {
    fn add(&self, x: i32, y: i32) -> i32;
}

default impl<T> Adder for T {
    fn add(&self, x: i32, y: i32) -> i32 {
        0
    }
}

impl Adder for () {
    fn add(&self, x: i32, y: i32) -> i32 {
        x + y
    }
}

fn main() {
    let adder = ();
    let x = adder.add(22, 44);
    assert_eq!(x, 66);
    println!("sum()={:?}", x);
}