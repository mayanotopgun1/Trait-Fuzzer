#![feature(impl_trait_in_assoc_type)]
#![allow(trivial_bounds)]

trait Bad {
    type Assert
    where
        Self: Sized;

    fn check(&self) -> impl core::fmt::Debug;
}

struct Tail([(); 0]);

impl Bad for Tail {
    type Assert = i32;

    fn check(&self) -> impl core::fmt::Debug {
        true
    }
}

#[cfg(any(bad, bad_new))]
const FOO: <Tail as Bad>::Assert = todo!();

#[cfg(any(good, good_new))]
trait BadExt: Bad {}
#[cfg(any(good, good_new))]
impl<T: Bad> BadExt for T {}

#[cfg(any(good, good_new))]
fn foo() where Tail: BadExt {
    let _: <Tail as Bad>::Assert;
}

fn main() {}