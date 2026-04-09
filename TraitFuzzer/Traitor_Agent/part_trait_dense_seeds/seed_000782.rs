#![feature(adt_const_params)]
#![allow(incomplete_features)]

use std::marker::ConstParamTy;

#[derive(PartialEq, Eq, ConstParamTy)]
struct Yikes;

trait MutSelf {
    fn mut_self(&mut self);
}

impl MutSelf for Yikes {
    fn mut_self(&mut self) {}
}

fn foo<const YIKES: Yikes>() {
    YIKES.mut_self()
}

fn main() {
    foo::<{ Yikes }>()
}