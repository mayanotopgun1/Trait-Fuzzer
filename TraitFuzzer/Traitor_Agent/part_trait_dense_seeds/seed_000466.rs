#![feature(fn_delegation)]
#![allow(incomplete_features)]
#![allow(dead_code)]

trait Trait1 {
    fn foo(&self) -> i32;
}

trait Trait2 {
    fn foo(&self) -> i32 { 2 }
}

struct F;
impl Trait1 for F {
    fn foo(&self) -> i32 { 1 }
}
impl Trait2 for F {}

impl F {
    fn foo(&self) -> i32 { 3 }
}

struct S(F);

impl Trait1 for S {
    fn foo(&self) -> i32 {
        self.0.foo()
    }
}

fn main() {
    let s = S(F);
    assert_eq!(s.foo(), 1);
}