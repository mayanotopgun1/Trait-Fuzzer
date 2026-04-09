#![deny(unknown_lints)]
#![allow(out_of_scope_macro_calls)]

trait Mainable {
    fn run(self);
}

impl Mainable for () {
    fn run(self) {}
}

fn main() {
    let _ = ().run();
}