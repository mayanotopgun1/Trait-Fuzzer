#![feature(specialization)]
#![allow(mixed_script_confusables, non_camel_case_types)]

trait Fooable<'β, γ> { fn foo(&self); }
default impl<'β, γ> Fooable<'β, γ> for () { fn foo(&self) {} }

struct X {
    δ: usize,
}

pub trait AlphaHolder { const α: f64; }

impl AlphaHolder for () { const α: f64 = 0.00001f64; }

pub fn main() {
    let _: () = ();
    let _ = <()>::α;
}