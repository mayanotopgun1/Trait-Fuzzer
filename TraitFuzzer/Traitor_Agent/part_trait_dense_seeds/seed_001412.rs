#![feature(generic_associated_types, type_alias_impl_trait)]

pub trait Trait<'a> {
    type Assoc;
}

pub struct S1<T>(T);
pub struct S2<T>(T);

pub type T1 = impl Trait<'static>;
pub type T2 = S1<T1>;
pub type T3 = S2<T2>;

impl<'a, T: 'a> Trait<'a> for S1<T> {
    type Assoc = ();
}

impl<'a, T: Trait<'a>> S2<T> {}

trait UseTrait<'a> {
    type Out;
    fn use_t(&self) -> Self::Out;
}

impl<'a, T: Trait<'a>> UseTrait<'a> for S1<T> {
    type Out = ();
    fn use_t(&self) -> Self::Out {}
}

impl<'a> UseTrait<'a> for T3 {
    type Out = ();
    fn use_t(&self) -> Self::Out {}
}

#[define_opaque(T1)]
pub fn use_t1() -> T1 {
    S1(())
}

fn main() {}