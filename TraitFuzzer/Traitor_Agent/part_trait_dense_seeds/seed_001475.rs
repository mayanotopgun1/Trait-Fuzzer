#![feature(generic_associated_types)]

trait BooleanValue {
    type Output<'a> where Self: 'a;
    fn value<'a>(&'a self) -> Self::Output<'a>;
}

trait BooleanDebug: BooleanValue {
    fn debug_value<'a>(&'a self) -> String where Self::Output<'a>: std::fmt::Debug {
        format!("{:?}", self.value())
    }
}

impl<T> BooleanDebug for T where T: BooleanValue {}

impl BooleanValue for bool {
    type Output<'a> = &'a bool;
    fn value<'a>(&'a self) -> Self::Output<'a> {
        self
    }
}

fn main() {
    let inappropriate_camel_casing = true;
    let _ = inappropriate_camel_casing.debug_value();
}