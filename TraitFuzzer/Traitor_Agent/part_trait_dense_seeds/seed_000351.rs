#![allow(dead_code)]
#![feature(impl_trait_in_assoc_type)]

trait A { fn dummy(&self) { } }

struct B<'a, T:'a> {
    f: &'a T
}

impl<'a, T> A for B<'a, T> {}

trait BExt<T>: A {
    type Out;
    fn inspect_f(&self) -> Self::Out;
}

impl<'a, T: std::fmt::Debug> BExt<T> for B<'a, T> {
    type Out = impl std::fmt::Debug;
    fn inspect_f(&self) -> Self::Out {
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