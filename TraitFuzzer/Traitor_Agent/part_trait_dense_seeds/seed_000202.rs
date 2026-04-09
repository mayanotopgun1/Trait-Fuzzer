#![feature(impl_trait_in_assoc_type)]
trait Foo {
    type Bar;
}

impl Foo for () {
    type Bar = ();
}

trait FooCopy<F: Foo> where F::Bar: Copy {}
impl<T: Foo<Bar = impl Copy>> FooCopy<T> for T {}

fn a<F: FooCopy<F> + Foo>() where <F as Foo>::Bar: Copy {}

fn b<F: FooCopy<F> + Foo>() where <F as Foo>::Bar: Copy {}

fn c<F: Foo<Bar: Foo>>() where F::Bar: Copy {}

fn main() {
    a::<()>();
    b::<()>();
    c::<()>();
}