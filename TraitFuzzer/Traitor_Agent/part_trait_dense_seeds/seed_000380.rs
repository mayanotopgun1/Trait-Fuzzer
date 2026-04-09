#![feature(return_position_impl_trait_in_trait)]
#![allow(dead_code)]

trait A { fn a(&self) -> isize; }
trait B: A { fn b(&self) -> isize; }
trait C: A { fn c(&self) -> isize; }

trait D: B + C {
    fn check_values(&self) -> impl Iterator<Item = bool> {
        vec![
            self.a() == 10,
            self.b() == 20,
            self.c() == 30,
        ].into_iter()
    }
}

impl<T> D for T where T: B + C {}

struct S { bogus: () }

impl A for S { fn a(&self) -> isize { 10 } }
impl B for S { fn b(&self) -> isize { 20 } }
impl C for S { fn c(&self) -> isize { 30 } }

fn f<T: D>(x: &T) {
    let checks = x.check_values().collect::<Vec<_>>();
    assert!(checks.iter().all(|&b| b));
}

pub fn main() {
    f(&S { bogus: () });
}