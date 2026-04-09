#![feature(unsized_fn_params)]

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
    fn index_mut(&mut self, index: &'a str) -> &mut Self::Output;
}

impl<'a> MutIndexable<'a> for A {
    fn index_mut(&mut self, _: &'a str) -> &mut Self::Output {
        panic!()
    }
}

fn main() {
    let a = A {};
    let s = String::new().into_boxed_str();
    assert_eq!(&(), a.index(&*s));
}