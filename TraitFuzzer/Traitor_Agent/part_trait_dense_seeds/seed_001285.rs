#![allow(path_statements)]

trait Outer<T> {
    fn outer(&mut self, t: T) -> impl core::fmt::Debug;
}

macro_rules! inner {
    ($e:pat ) => ($e)
}

macro_rules! outer {
    ($e:pat ) => (inner!($e))
}

impl Outer<i32> for i32 {
    fn outer(&mut self, t: i32) -> impl core::fmt::Debug {
        *self = t;
        "Operation completed"
    }
}

fn main() {
    let mut g1 = 0;
    let result = g1.outer(13);
    println!("{:?}", result);
}