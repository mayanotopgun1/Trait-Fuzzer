trait Supertrait<T> {
    fn method(&self) {}
}
impl<T> Supertrait<T> for () {}

trait Identity {
    type Selff;
}
impl<Selff> Identity for Selff {
    type Selff = Selff;
}

trait Trait<P>: Supertrait<()> + Supertrait<<P as Identity>::Selff> {
    fn trait_method(&self) {}
}

impl<P> Trait<P> for () {
    fn trait_method(&self) {}
}

fn main() {
    let x: &dyn Trait<()> = &();
    let _: &dyn Supertrait<()> = x;
    x.trait_method();
}