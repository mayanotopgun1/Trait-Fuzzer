#![deny(unused_crate_dependencies)]

trait Mainable {
    fn execute();
}

impl Mainable for () {
    fn execute() {}
}

fn main() {
    <() as Mainable>::execute();
}