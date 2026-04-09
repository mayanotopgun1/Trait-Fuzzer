#![feature(generic_associated_types)]
#![allow(warnings)]

struct Struct {
    a: usize,
}

trait Alike<'a> {
    type AssocType;
    fn get_a(&'a self) -> Self::AssocType;
}

trait ExtendedAlike<'a>: Alike<'a> {
    fn double_get_a(&'a self) -> (Self::AssocType, Self::AssocType) {
        let a = self.get_a();
        (a, self.get_a())
    }
}

impl<'a, T: Alike<'a>> ExtendedAlike<'a> for T {}

impl<'a> Alike<'a> for Struct {
    type AssocType = &'a usize;
    fn get_a(&'a self) -> Self::AssocType {
        &self.a
    }
}

const C: usize = 1;
static S: usize = 1;

const T1: &'static usize = &C;
const T2: &'static usize = &S;
static T3: &'static usize = &C;
static T4: &'static usize = &S;

const T5: usize = C;
const T6: usize = S;
static T7: usize = C;
static T8: usize = S;

const T9: Struct = Struct { a: C };
const T10: Struct = Struct { a: S };

static T11: Struct = Struct { a: C };
static T12: Struct = Struct { a: S };

fn main() {
    let _ = T9.double_get_a();
    let _ = T10.double_get_a();
    let _ = T11.double_get_a();
    let _ = T12.double_get_a();
}