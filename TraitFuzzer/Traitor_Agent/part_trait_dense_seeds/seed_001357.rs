#![allow(non_fmt_panics)]
#![feature(unboxed_closures)]

trait UnreachableExt {
    fn unreachable_with_value(self: Box<Self>, message: &'static str);
}

impl<T: std::fmt::Display + 'static> UnreachableExt for T {
    fn unreachable_with_value(self: Box<Self>, message: &'static str) {
        unreachable!("{}", format_args!("{} is {}", message, *self))
    }
}

fn main() {
    let x = 5;
    Box::new(x).unreachable_with_value("x");
}