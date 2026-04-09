#![feature(generic_associated_types, adt_const_params, unsized_const_params, generic_const_exprs)]
#![allow(incomplete_features, unused_variables)]

struct F<const S: &'static str>;
impl<const S: &'static str> X<{ S }> for F<{ S }> {
    type T<'a> = &'a [u8; Self::W] where Self: 'a, [(); Self::W]:;

    const W: usize = 3;

    fn d(r: &[u8; Self::W]) -> F<{ S }> {
        let x: [u8; Self::W] = [0; Self::W];
        F
    }

    fn create_default() -> Self where [(); Self::W]: {
        let default_data: [u8; Self::W] = [0; Self::W];
        Self::d(&default_data)
    }
}

pub trait X<const S: &'static str>: Sized {
    type T<'a> where Self: 'a, [(); Self::W]:;

    const W: usize;
    fn d(r: &[u8; Self::W]) -> Self;

    fn create_default() -> Self where [(); Self::W]: {
        let default_data: [u8; Self::W] = [0; Self::W];
        Self::d(&default_data)
    }
}

trait XDefault<const S: &'static str>: X<{ S }> {
    fn default_value() -> Self where [(); Self::W]: {
        Self::create_default()
    }
}

impl<const S: &'static str, T: X<{ S }>> XDefault<{ S }> for T {}

fn main() {}