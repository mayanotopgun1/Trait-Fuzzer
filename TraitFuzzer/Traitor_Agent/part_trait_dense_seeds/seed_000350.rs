#![allow(dead_code)]

trait A { fn dummy(&self) { } }

struct B<'a, T:'a> {
    f: &'a T
}

impl<'a, T> A for B<'a, T> {}

trait BExt<T>: A {
    fn inspect_f(&self) -> &T;
}

impl<'a, T> BExt<T> for B<'a, T> {
    fn inspect_f(&self) -> &T {
        self.f
    }
}

fn foo(_: &dyn A) {}

fn bar<G: std::fmt::Debug>(b: &B<G>) {
    foo(b);
    foo(b as &dyn A);
    println!("{:?}", b.inspect_f());
}

fn main() {}