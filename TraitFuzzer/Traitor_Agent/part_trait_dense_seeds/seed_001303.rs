#![feature(generic_associated_types, unsized_fn_params)]

use std::ops;

pub struct A;

trait Indexable<'a> {
    type Output;
    fn index(&self, index: &'a str) -> &Self::Output;
}

impl<'a> Indexable<'a> for A {
    type Output = ();
    fn index(&self, _: &'a str) -> &Self::Output {
        &()
    }
}

trait MutIndexable<'a>: Indexable<'a> {
    type Out<'b>: 'b where Self: 'b;
    fn index_mut<'b>(&mut self, index: &'b str) -> Self::Out<'b>;
}

impl<'a> MutIndexable<'a> for A {
    type Out<'b> = &'b mut ();
    fn index_mut<'b>(&mut self, _: &'b str) -> Self::Out<'b> {
        panic!()
    }
}

fn main() {
    let a = A {};
    let s = String::new().into_boxed_str();
    assert_eq!(&(), a.index(&*s));
}