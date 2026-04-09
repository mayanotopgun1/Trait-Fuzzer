trait Sup<T> {
    type Assoc;
}

impl<T> Sup<T> for () {
    type Assoc = T;
}

trait Trait<A, B>: Sup<A, Assoc = A> + Sup<B, Assoc = B> {}

impl<T, U> Trait<T, U> for () {}

trait TraitExt: Trait<(), ()> {}
impl<T> TraitExt for T where T: Trait<(), ()> {}

fn main() {
    let x: &dyn TraitExt = &();
    let y: &dyn Trait<_, ()> = x;
}