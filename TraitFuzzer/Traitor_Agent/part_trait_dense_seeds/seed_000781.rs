#![feature(specialization)]
#![allow(dead_code)]

trait Access {
    fn get_ref(&self) -> &'static dyn core::fmt::Debug;
}

default impl<T> Access for T {
    default fn get_ref(&self) -> &'static dyn core::fmt::Debug {
        &() as &'static dyn core::fmt::Debug
    }
}

struct A {
    a: &'static (),
}

impl Access for A {
    fn get_ref(&self) -> &'static dyn core::fmt::Debug {
        self.a as &'static dyn core::fmt::Debug
    }
}

trait AccessExt: Access {
    fn ref_twice(&self) -> (&'static dyn core::fmt::Debug, &'static dyn core::fmt::Debug) {
        (self.get_ref(), self.get_ref())
    }
}

impl<T: Access> AccessExt for T {}

static B: &'static A = &A { a: &() };
static C: &'static A = &B;

fn main() {}