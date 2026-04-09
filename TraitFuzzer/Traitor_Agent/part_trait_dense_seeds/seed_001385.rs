use std::marker::PhantomData;

pub struct InvariantRef<'a, T: ?Sized>(&'a T, PhantomData<&'a mut &'a T>);

trait RefTrait<'a, T: ?Sized> {
    fn new(r: &'a T) -> Self;
}

impl<'a, T: ?Sized> RefTrait<'a, T> for InvariantRef<'a, T> {
    fn new(r: &'a T) -> Self {
        InvariantRef(r, PhantomData)
    }
}

fn issue_78174() {
    let foo = const { "foo" };
    assert_eq!(foo, "foo");
}

fn get_invariant_ref<'a>() -> InvariantRef<'a, ()> {
    RefTrait::new(&())
}

fn get_invariant_ref2<'a>() -> InvariantRef<'a, ()> {
    InvariantRef::new(&())
}

fn main() {
    issue_78174();
    get_invariant_ref();
    get_invariant_ref2();
}