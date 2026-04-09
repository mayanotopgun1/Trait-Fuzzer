use std::fmt;

trait DebugExt: fmt::Debug {
    fn debug_info(&self) -> String {
        format!("{:?}", self)
    }
}

impl<T: fmt::Debug> DebugExt for T {}

fn main() {
    let a: &dyn DebugExt = &1;
    let _ = a.debug_info();
}