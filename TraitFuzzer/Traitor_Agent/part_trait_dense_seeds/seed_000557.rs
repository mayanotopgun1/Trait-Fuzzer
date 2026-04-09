trait B<C> {}
impl<C> B<C> for () {}

trait ExtendedB<C>: B<C> {
    fn extended_f(&self) {}
}

impl<C> ExtendedB<C> for () where Self: B<C> {}

trait D<C, E>: B<C> + B<E> + ExtendedB<C> + ExtendedB<E> {
    fn f(&self) {}
}

impl<C, E> D<C, E> for () where Self: B<C> + B<E> + ExtendedB<C> + ExtendedB<E> {}

fn main() {
    let x = &() as &dyn D<&(), &()>;
    x.f();
}