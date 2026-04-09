#![feature(specialization)]
#![allow(dead_code)]

struct A<'a> {
    a: &'a i32,
    b: &'a i32,
}

trait CloneA<'a>: Sized {
    fn clone_a(&self) -> Self;
}

default impl<'a, T> CloneA<'a> for T
where
    T: Clone,
{
    fn clone_a(&self) -> Self {
        self.clone()
    }
}

impl<'a> CloneA<'a> for A<'a> {
    fn clone_a(&self) -> Self {
        A {
            a: self.a,
            b: self.b,
        }
    }
}

fn main() {}