#![allow(stable_features)]
#![feature(rust1)]

trait MainRunner {
    fn run(&self);
}

trait MainRunnerExt: MainRunner {
    fn run_twice(&self) {
        self.run();
        self.run();
    }
}

impl<T: MainRunner> MainRunnerExt for T {}

impl MainRunner for () {
    fn run(&self) {}
}

fn main() {
    let _ = ().run_twice();
}