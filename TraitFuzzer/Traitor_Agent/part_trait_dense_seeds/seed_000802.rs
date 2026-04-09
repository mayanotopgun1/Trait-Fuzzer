#![feature(generic_associated_types)]
#![allow(unused_must_use)]
#![allow(unused_imports)]
#![allow(deprecated)]

use std::hash::{Hash, SipHasher, Hasher};

#[derive(PartialEq, Clone, Hash, Debug)]
struct Foo {
    bar: usize,
    baz: isize
}

trait PartialEqSelf {
    fn equal(&self, other: &Self) -> bool;
}

impl PartialEqSelf for Foo {
    fn equal(&self, other: &Self) -> bool {
        self == other
    }
}

trait CloneSelf {
    type Out;
    fn duplicate(&self) -> Self::Out;
}

impl CloneSelf for Foo {
    type Out = Foo;
    fn duplicate(&self) -> Self::Out {
        self.clone()
    }
}

trait Hashable<'a> {
    type Output;
    fn hash_value(&'a self) -> Self::Output;
}

impl<'a> Hashable<'a> for Foo {
    type Output = u64;
    fn hash_value(&'a self) -> Self::Output {
        let mut hasher = SipHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}

trait ExtendedFoo: PartialEqSelf + CloneSelf + for<'b> Hashable<'b> {}

impl ExtendedFoo for Foo {}

fn hash<T: for<'b> Hashable<'b>>(_t: &T) {}

pub fn main() {
    let a = Foo {bar: 4, baz: -3};

    a.equal(&a);
    let _b = a.duplicate();
    hash(&a);
}