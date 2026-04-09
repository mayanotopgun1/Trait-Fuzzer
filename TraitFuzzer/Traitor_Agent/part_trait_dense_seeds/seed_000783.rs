#![feature(adt_const_params, const_trait_impl)]
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

fn foo(mut x: Box<dyn MutSelf>) {
    x.mut_self();
}

fn main() {
    let yikes = Yikes;
    foo(Box::new(yikes));
}