#![expect(warnings)]

trait MainRunner {
    fn run(&self);
}

trait MainRunnerExt: MainRunner {
    fn run_with_info(&self, info: &str) where Self: Sized {
        println!("Running with info: {}", info);
        self.run();
    }
}

impl<T> MainRunnerExt for T where T: MainRunner {}

impl MainRunner for () {
    fn run(&self) {}
}

fn main() {
    let x: () = ();
    <() as MainRunnerExt>::run_with_info(&x, "No-op operation");
}