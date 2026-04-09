trait TraitA<'a> {
    type AsA;
}

trait TraitB<'a, 'b> {
    type AsB;
}

trait TraitC<'a, 'b, 'c> {}

struct X;

impl<'a, 'b, 'c> TraitC<'a, 'b, 'c> for X {}

struct Y;

impl<'a, 'b> TraitB<'a, 'b> for Y {
    type AsB = X;
}

struct Z;

impl<'a> TraitA<'a> for Z {
    type AsA = Y;
}

trait ExtendedTraitC<'a, 'b, 'c>: TraitC<'a, 'b, 'c> {
    fn check(&self) -> bool { true }
}

impl<'a, 'b, 'c, T> ExtendedTraitC<'a, 'b, 'c> for T where T: TraitC<'a, 'b, 'c> {}

trait ExtendedTraitB<'a, 'b>: TraitB<'a, 'b> {
    fn inspect(&self) -> bool { true }
}

impl<'a, 'b, T> ExtendedTraitB<'a, 'b> for T where T: TraitB<'a, 'b> {}

trait ExtendedTraitA<'a>: TraitA<'a> {
    fn verify(&self) -> bool { true }
}

impl<'a, T> ExtendedTraitA<'a> for T where T: TraitA<'a> {}

fn foo<T>()
where
    for<'a> T: ExtendedTraitA<'a, AsA: for<'b> ExtendedTraitB<'a, 'b, AsB: for<'c> ExtendedTraitC<'a, 'b, 'c>>>,
{
}

fn main() {
    foo::<Z>();
}