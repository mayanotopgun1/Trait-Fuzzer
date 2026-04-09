#![feature(type_alias_impl_trait)]
#![allow(incomplete_features)]
#![allow(dead_code)]

trait Trait1 {
    type Out;
    fn foo(&self) -> Self::Out;
}

trait Trait2 {
    fn foo(&self) -> i32 { 2 }
}

struct F;
impl Trait1 for F {
    type Out = i32;
    fn foo(&self) -> Self::Out { 1 }
}
impl Trait2 for F {}

type HiddenFoo = i32;
impl F {
    fn foo(&self) -> HiddenFoo { 3 }
}

struct S(F);

impl Trait1 for S {
    type Out = i32;
    fn foo(&self) -> Self::Out {
        self.0.foo()
    }
}

fn main() {
    let s = S(F);
    assert_eq!(s.foo(), 1);
}