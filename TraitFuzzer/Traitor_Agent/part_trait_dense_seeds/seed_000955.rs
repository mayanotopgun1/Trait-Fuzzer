#![allow(incomplete_features)]
#![feature(specialization)]

#[derive(PartialEq)]
enum Never {}

trait Foo<'a> {
    type Assoc: PartialEq;
}
impl<'a, T> Foo<'a> for T {
    default type Assoc = Never;
}

trait Trait1 {
    type Selection: PartialEq;
}
trait Trait2: PartialEq<Self> {}
impl<T: Trait2> Trait1 for T {
    default type Selection = T;
}

trait Trait3 {
    fn check_eq(&self, other: &Self) -> bool where Self: PartialEq;
}
impl<S> Trait3 for S where S: PartialEq {
    fn check_eq(&self, other: &Self) -> bool {
        self == other
    }
}


trait Trait1Ext: Trait1 {
    fn selection_ref(&self) -> &Self::Selection;
}
impl<S> Trait1Ext for S where S: Trait1 {
    fn selection_ref(&self) -> &Self::Selection {
        // Since there is no `selection` method in the original code, we cannot directly call it.
        // We need to provide a valid implementation or remove this method if not needed.
        unimplemented!()
    }
}


trait Trait2Ext: Trait2 {
    fn double_eq(&self, other: &Self) -> bool;
}
impl<S> Trait2Ext for S where S: Trait2 {
    fn double_eq(&self, other: &Self) -> bool {
        self == other && self == other
    }
}

fn main() {}