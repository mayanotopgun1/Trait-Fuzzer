#![feature(type_alias_impl_trait)]
#![feature(impl_trait_in_assoc_type)]

trait Foo: Sized {
    fn foo(self) {}
}

trait Bar: Foo + Sized {
    type Output;
    fn bar(self) -> Self::Output;
}

struct S;

impl<'l> Foo for &'l S {}

impl<T: Foo> Bar for T {
    type Output = &'static str;
    fn bar(self) -> Self::Output { "bar" }
}

fn main() {
    let s = S;
    s.foo();
    (&s).bar();
    s.bar();
}