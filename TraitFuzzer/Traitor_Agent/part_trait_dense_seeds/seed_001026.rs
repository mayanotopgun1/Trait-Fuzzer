#![feature(impl_trait_in_assoc_type)]

pub enum E<'a> {
    Empty,
    Some(&'a E<'a>),
}

trait Fuzzable {
    type Out: core::fmt::Debug + PartialEq<u32>;
    fn fuzz(&self) -> Self::Out;
}

impl<'a> Fuzzable for E<'a> {
    type Out = u32;

    fn fuzz(&self) -> Self::Out {
        if let E::Some(E::Some(_)) = self { 1 } else { 2 }
    }
}

fn main() {
    assert_eq!(E::Empty.fuzz(), 2);
}