#![feature(rustc_attrs)]
#![allow(unused)]

use std::borrow::Borrow;
use std::ops::Deref;

struct PlainType<T>(T);

#[derive(Clone)]
struct CloneType<T>(T);

trait CloneExt: Clone {
    fn clone_ref(&self) -> Self { self.clone() }
}

impl<T: Clone> CloneExt for T {}

fn check(encoded: &[u8]) -> impl Iterator<Item = u8> + '_ {
    encoded.iter().cloned()
}

fn main() {
    let non_clone_type_ref = &PlainType(1u32);
    let non_clone_type_ref_clone: &PlainType<u32> = non_clone_type_ref.clone_ref();

    let clone_type_ref = &CloneType(1u32);
    let clone_type_ref_clone: CloneType<u32> = clone_type_ref.clone();

    let non_deref_type = &PlainType(1u32);
    let non_deref_type_deref: &PlainType<u32> = non_deref_type.deref();

    let non_borrow_type = &PlainType(1u32);
    let non_borrow_type_borrow: &PlainType<u32> = non_borrow_type.borrow();

    let non_borrow_type = &&PlainType(1u32);
    let non_borrow_type_borrow: &PlainType<u32> = non_borrow_type.borrow();
}

fn generic<T>(non_clone_type: &PlainType<T>) where T: Clone {
    non_clone_type.clone_ref();
}

fn non_generic(non_clone_type: &PlainType<u32>) {
    non_clone_type.clone_ref();
}

struct DiagnosticClone;
impl Clone for DiagnosticClone {
    #[rustc_diagnostic_item = "other_clone"]
    fn clone(&self) -> Self {
        DiagnosticClone
    }
}

trait CloneDiagnosticExt {
    fn diagnostic_clone(&self) -> Self where Self: Sized + Clone;
}

impl<T: Clone> CloneDiagnosticExt for T {
    fn diagnostic_clone(&self) -> Self { self.clone() }
}

fn with_other_diagnostic_item(x: DiagnosticClone) {
    x.diagnostic_clone();
}