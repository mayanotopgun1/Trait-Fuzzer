#![allow(unused_variables)]
#![feature(impl_trait_in_assoc_type)]

use std::ops::Index;
use std::fmt::{Debug, Display};

trait Indexable {
    type Output: ?Sized;

    fn index<'a>(&'a self, idx: usize) -> &'a Self::Output;
}

impl Indexable for S {
    type Output = str;

    fn index<'a>(&'a self, _: usize) -> &'a Self::Output {
        "hello"
    }
}

impl Indexable for T {
    type Output = dyn Debug + 'static;

    fn index<'a>(&'a self, idx: usize) -> &'a Self::Output {
        static X: usize = 42;
        &X as &(dyn Debug + 'static)
    }
}

struct S;

struct T;

impl Index<usize> for S {
    type Output = str;

    fn index<'a>(&'a self, idx: usize) -> &'a Self::Output {
        Indexable::index(self, idx)
    }
}

impl Index<usize> for T {
    type Output = dyn Debug + 'static;

    fn index<'a>(&'a self, idx: usize) -> &'a Self::Output {
        Indexable::index(self, idx)
    }
}

fn main() {
    assert_eq!(&S[0], "hello");
    let _ = &T[0];
}