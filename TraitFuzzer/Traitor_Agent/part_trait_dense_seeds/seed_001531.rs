#![allow(dead_code)]

trait T0 {
    type O;
    fn dummy(&self) { }
}

struct S<A>(A);
impl<A> T0 for S<A> { type O = A; }

trait T1: T0 {

    fn m0<F: Fn(<Self as T0>::O) -> bool>(&self, f: F) -> bool;
}

trait T1Ext: T1 {
    fn m0_twice<F: Fn(<Self as T0>::O) -> bool>(&self, f: F) -> bool {
        self.m0(&f) && self.m0(&f)
    }
}

impl<T> T1Ext for T where T: T1 {}

impl<A: Clone> T1 for S<A> {
    fn m0<F: Fn(<Self as T0>::O) -> bool>(&self, f: F) -> bool { f(self.0.clone()) }
}

fn main() { }