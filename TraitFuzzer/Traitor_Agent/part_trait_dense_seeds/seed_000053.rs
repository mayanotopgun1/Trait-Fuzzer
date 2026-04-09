#![feature(impl_trait_in_assoc_type)]

macro_rules! w {
    ($($tt:tt)*) => {};
}

trait LifetimeMacroExt { fn macro_w(&self); }
impl<S> LifetimeMacroExt for S {
    fn macro_w(&self) {}
}

fn main() {}