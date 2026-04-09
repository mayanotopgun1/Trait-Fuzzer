#![feature(impl_trait_in_assoc_type)]

trait Fooable {
    fn foo(self);
}

impl<F> Fooable for F
where
    F: 'static,
{
    fn foo(self) {}
}

trait Fromable {
    type Out;
    fn from(self) -> Self::Out;
}

impl<F> Fromable for F
where
    F: Send + std::fmt::Debug,
{
    type Out = impl core::fmt::Debug + Send;
    fn from(self) -> Self::Out {
        self
    }
}

fn bar<T>() {
    let f = || ();
    let _ = Fooable::foo(Fromable::from(f()));
}

fn main() {}