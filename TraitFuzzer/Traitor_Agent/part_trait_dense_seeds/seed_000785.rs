#![allow(dead_code)]
#![feature(specialization)]

trait Foo<T> {
    fn noop(&self, _: T);
}

default impl<S, T> Foo<T> for S where T: Copy {
    fn noop(&self, _: T) {}
}

trait NoopTwice<T>: Foo<T> where T: Copy {
    fn noop_twice(&self, x: T) { self.noop(x); }
}

impl<S, T> NoopTwice<T> for S where S: Foo<T>, T: std::marker::Copy {}

enum Bar<T> { Bla(T) }

struct Baz<'a> {
    inner: dyn for<'b> Foo<Bar<&'b ()>> + 'a,
}

fn main() {}