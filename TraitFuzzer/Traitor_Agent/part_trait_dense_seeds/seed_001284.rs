#![allow(path_statements)]

trait Outer<T> {
    fn outer(&mut self, t: T);
}

macro_rules! inner {
    ($e:pat ) => ($e)
}

macro_rules! outer {
    ($e:pat ) => (inner!($e))
}

impl Outer<i32> for i32 {
    fn outer(&mut self, t: i32) {
        *self = t;
    }
}

fn main() {
    let mut g1 = 0;
    g1.outer(13);
    g1;
}