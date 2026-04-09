#![feature(impl_trait_in_assoc_type)]

#[cfg(a)]
trait Fooable {
    type AssocType;
    fn foo() -> Self::AssocType;
}

#[cfg(a)]
impl Fooable for () {
    type AssocType = impl Into<u32>;
    fn foo() -> Self::AssocType {
        22_usize
    }
}

fn main() {}