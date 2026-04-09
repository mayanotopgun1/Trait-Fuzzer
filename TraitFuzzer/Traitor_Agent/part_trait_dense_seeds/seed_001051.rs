use std::fmt::Debug;
use std::fmt::Formatter;

#[derive(Debug)]
struct X;

trait DebugExt: Debug {
    fn debug_ext(&self) -> String {
        format!("{:?}", self)
    }
}

impl<T: Debug> DebugExt for T {}

fn main() {
    let x = X;
    dbg!(x.debug_ext());
}