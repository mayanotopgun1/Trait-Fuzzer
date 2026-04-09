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
    F: Send,
{
    type Out = F;
    fn from(self) -> Self::Out {
        self
    }
}

fn bar<T>() {
    let f = || ();
    let _ = Fooable::foo(Fromable::from(f));
}

fn main() {}