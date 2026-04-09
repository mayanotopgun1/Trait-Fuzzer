#![feature(impl_trait_in_assoc_type)]

struct A<'a>(&'a ());

trait ArrayInitializer<const N: usize> {
    type Output;
    fn initialize_array(&self) -> Self::Output;
}

impl<'a> ArrayInitializer<68> for A<'a> {
    type Output = impl core::fmt::Debug;

    fn initialize_array(&self) -> Self::Output {
        let mut b = [0; 68];
        b
    }
}

fn main() {}