#![feature(type_alias_impl_trait)]
#![feature(min_generic_const_args)]
#![expect(incomplete_features)]

trait Trait<const N: usize> {}

trait GenericTrait<const N: usize>: Trait<N> {
    type Alias;
    const FN_IDENTITY: for<'a> fn(&'a ());
}

struct ConcreteAlias;

impl core::fmt::Debug for ConcreteAlias {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ConcreteAlias")
    }
}

type HiddenAlias = ConcreteAlias;

trait GenericTraitExt<const N: usize>: GenericTrait<N> {
    fn identity_call(&self) {}
}

impl<T, const N: usize> GenericTraitExt<N> for T
where
    T: GenericTrait<N>,
{}

impl<const N: usize, T> GenericTrait<N> for T
where
    T: Trait<N>,
{
    type Alias = HiddenAlias;
    const FN_IDENTITY: for<'a> fn(&'a ()) = |_: &()| {};
}

fn foo<T>()
where
    T: GenericTrait<1> + GenericTraitExt<1>,
{}

fn main() {}