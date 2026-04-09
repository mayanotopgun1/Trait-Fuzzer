#![feature(adt_const_params, unsized_const_params, generic_const_exprs, impl_trait_in_assoc_type)]

trait Combine<const CHANGES: &'static [&'static str]> where [(); CHANGES.len()]: {
    fn combine(&mut self, other: &Self) -> impl core::fmt::Debug;
}

pub struct Changes<const CHANGES: &'static [&'static str]>
where
    [(); CHANGES.len()]:,
{
    changes: [usize; CHANGES.len()],
}

impl<const CHANGES: &'static [&'static str]> Combine<CHANGES> for Changes<CHANGES>
where
    [(); CHANGES.len()]:,
{
    fn combine(&mut self, other: &Self) -> impl core::fmt::Debug {
        self.changes.iter().zip(other.changes.iter()).map(|(a, b)| a + b).collect::<Vec<_>>()
    }
}

pub fn main() {}