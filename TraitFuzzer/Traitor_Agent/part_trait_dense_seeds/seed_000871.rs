#![feature(rustc_attrs, marker_trait_attr)]
#[rustc_coinductive]
trait Trait {}

impl<T, U> Trait for (T, U)
where
    (U, T): Trait,
    (T, U): Inductive,
    (): ConstrainToU32<T>,
{}

trait TraitExt: Trait {}
impl<T, U> TraitExt for (T, U)
where
    (U, T): Trait,
    (T, U): Inductive,
    (): ConstrainToU32<T>,
{
}

trait ConstrainToU32<T> {}
impl ConstrainToU32<u32> for () {}

#[rustc_coinductive]
trait Inductive {}
impl<T, U> Inductive for (T, U)
where
    T: Inductive,
    U: Inductive,
{}
impl Inductive for u32 {}

fn impls_trait_ext<T, U>()
where
    (T, U): TraitExt,
{}

fn main() {
    impls_trait_ext::<u32, u32>();
}