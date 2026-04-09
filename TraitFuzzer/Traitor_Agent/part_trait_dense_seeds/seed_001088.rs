#![feature(type_alias_impl_trait, impl_trait_in_assoc_type)]

trait Trait {
    type Opaque;
    fn provided() -> Self::Opaque;
}

struct Type;

impl Type {
    pub fn perform() -> impl core::fmt::Debug {
        42u32
    }
}

type Hidden = <Type as Trait>::Opaque;

impl Trait for Type {
    type Opaque = impl core::fmt::Debug;
    fn provided() -> Self::Opaque {
        Type::perform()
    }
}

fn main() {
    let _ = Type::provided();
}