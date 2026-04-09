trait MainRunner {
    type Item;

    fn run(&self);
}

trait MainRunnerExt: MainRunner {
    fn double_run(&self) {
        self.run();
        self.run();
    }
}

impl<T: ?Sized + MainRunner> MainRunnerExt for T {}

impl MainRunner for () {
    type Item = &'static ();

    fn run(&self) {}
}

fn main() {
    let runner: &dyn MainRunner<Item = &'static ()> = &();
    runner.double_run();
}