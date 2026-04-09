#![feature(specialization)]
#![allow(dead_code)]

trait Access {
    fn get_ref(&self) -> &'static ();
}

default impl<T> Access for T {
    default fn get_ref(&self) -> &'static () {
        &()
    }
}

struct A {
    a: &'static (),
}

impl Access for A {
    fn get_ref(&self) -> &'static () {
        self.a
    }
}

trait AccessExt: Access {
    fn ref_twice(&self) -> (&'static (), &'static ()) {
        (self.get_ref(), self.get_ref())
    }
}

impl<T: Access> AccessExt for T {}

static B: &'static A = &A { a: &() };
static C: &'static A = &B;

fn main() {}