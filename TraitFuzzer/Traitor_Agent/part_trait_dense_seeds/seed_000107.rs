#![feature(type_alias_impl_trait)]

use std::fmt;

trait DebugExt: fmt::Debug {
    fn debug_info(&self) -> String {
        format!("{:?}", self)
    }
}

impl<T: fmt::Debug> DebugExt for T {}

type DynDebugExt = dyn DebugExt + 'static;

fn main() {
    let a: &DynDebugExt = &1;
    let _ = a.debug_info();
}