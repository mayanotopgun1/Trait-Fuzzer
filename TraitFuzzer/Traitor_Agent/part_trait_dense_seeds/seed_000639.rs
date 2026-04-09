#![allow(warnings)]

trait Functionality {
    fn execute(&self);
}

impl Functionality for () {
    fn execute(&self) {}
}

fn main() {
    let _ = ().execute();
}