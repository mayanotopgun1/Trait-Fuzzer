#![allow(non_fmt_panics)]

trait UnreachableExt {
    fn unreachable_with_value(self, message: &'static str);
}

impl<T: std::fmt::Display> UnreachableExt for T {
    fn unreachable_with_value(self, message: &'static str) {
        unreachable!("{}", format_args!("{} is {}", message, self))
    }
}

fn main() {
    let x = 5;
    x.unreachable_with_value("x");
}