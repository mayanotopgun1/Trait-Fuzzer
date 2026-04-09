#![feature(const_trait_impl)]

const trait SumTuple {
    fn sum(self) -> i32;
}

impl const SumTuple for (i32, i32) {
    fn sum(self) -> i32 {
        let (a, b) = self;
        a + b
    }
}

const trait SumArray {
    fn sum(self) -> i32;
}

impl const SumArray for [i32; 2] {
    fn sum(self) -> i32 {
        let [a, b] = self;
        a + b
    }
}

fn main() {}

const fn tup(t: (i32, i32)) -> i32 {
    t.sum()
}

const fn array(a: [i32; 2]) -> i32 {
    a.sum()
}