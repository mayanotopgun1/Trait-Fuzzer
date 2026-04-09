#![feature(impl_trait_in_assoc_type)]

macro_rules! m {
    () => { #[cfg(false)] fn f() {} }
}

trait T {}
trait TM: T {
    type Assoc;
    m!();
}
impl<S> TM for S where S: T {
    type Assoc = ();
}

impl T for () {}

fn main() {}