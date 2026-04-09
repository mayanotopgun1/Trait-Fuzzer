#![feature(specialization)]

trait MainFn {
    fn run();
}

default impl<T> MainFn for T {
    fn run() {}
}

impl MainFn for () {
    fn run() {}
}

fn main() {
    <() as MainFn>::run();
}