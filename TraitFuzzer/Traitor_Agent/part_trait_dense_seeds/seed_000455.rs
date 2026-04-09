#![feature(generic_associated_types)]

trait MainRunner {
    type Output<'a> where Self: 'a;
    fn run<'a>(&'a self) -> Self::Output<'a>;
}

trait RunnerExt: MainRunner {
    fn double_run<'a>(&'a self) -> (Self::Output<'a>, Self::Output<'a>) {
        let first = self.run();
        let second = self.run(); // Call run again to get a fresh value
        (first, second)
    }
}

impl<T> RunnerExt for T where T: MainRunner {}

impl MainRunner for () {
    type Output<'a> = &'a ();
    fn run<'a>(&'a self) -> Self::Output<'a> { self }
}

fn main() {
    let runner: () = ();
    let _ = runner.double_run();
}