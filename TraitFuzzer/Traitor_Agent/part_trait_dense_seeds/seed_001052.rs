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

fn create_x() -> impl DebugExt {
    X
}

fn main() {
    let x = create_x();
    dbg!(x.debug_ext());
}