#![allow(dead_code)]

trait A { fn a(&self) -> isize; }
trait B: A { fn b(&self) -> isize; }
trait C: A { fn c(&self) -> isize; }

trait D: B + C {
    fn check_values(&self) {
        assert_eq!(self.a(), 10);
        assert_eq!(self.b(), 20);
        assert_eq!(self.c(), 30);
    }
}

impl<T> D for T where T: B + C {}

struct S { bogus: () }

impl A for S { fn a(&self) -> isize { 10 } }
impl B for S { fn b(&self) -> isize { 20 } }
impl C for S { fn c(&self) -> isize { 30 } }

fn f<T: D>(x: &T) {
    x.check_values();
}

pub fn main() {
    f(&S { bogus: () });
}