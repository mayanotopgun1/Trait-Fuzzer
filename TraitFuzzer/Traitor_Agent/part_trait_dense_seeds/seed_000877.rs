#![feature(generic_associated_types)]

trait Fooable { fn foo(self); }
impl<T> Fooable for T { fn foo(self) {} }

struct A<'a, B: 'a>(&'a B);

trait HasLifetime<'a> {}
impl<'a, B: 'a> HasLifetime<'a> for A<'a, B> {}

fn foo<T>(_t: T)
where
    T: Fooable,
{
}

fn main() {
    let (a1, a2): (String, A<_>) = (String::from("auto"), A(&"this"));
    foo((a1, a2));
}