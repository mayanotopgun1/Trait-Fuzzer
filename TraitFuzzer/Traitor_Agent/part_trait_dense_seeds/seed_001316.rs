#![allow(stable_features)]
#![feature(rust1, generic_associated_types)]

trait MainRunner {
    type Output<'a> where Self: 'a;
    fn run(&self) -> Self::Output<'static>;
}

trait MainRunnerExt: MainRunner {
    fn run_twice(&self) -> (Self::Output<'static>, Self::Output<'static>) {
        (self.run(), self.run())
    }
}

impl<T: MainRunner> MainRunnerExt for T {}

impl MainRunner for () {
    type Output<'a> = &'a ();
    fn run(&self) -> Self::Output<'static> { &() }
}

fn main() {
    let _ = ().run_twice();
}