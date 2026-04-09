#![feature(type_alias_impl_trait)]

#[macro_export]
macro_rules! generate_trait_2015_ident {
    ($Type: ident) => {
        trait Trait1 {
            type Out;
            fn method(self) -> Self::Out;
        }

        impl Trait1 for $Type {
            type Out = impl core::fmt::Debug;
            fn method(self) -> Self::Out {}
        }
    };
}

#[macro_export]
macro_rules! generate_trait_2015_tt {
    ($Type: tt) => {
        trait Trait2 {
            type Out;
            fn method(self) -> Self::Out;
        }

        impl Trait2 for $Type {
            type Out = impl core::fmt::Debug;
            fn method(self) -> Self::Out {}
        }
    };
}

fn main() {}