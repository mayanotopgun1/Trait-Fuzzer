trait MainRunner {
    fn run(&self);
}

impl MainRunner for () {
    fn run(&self) {}
}

fn main() {
    let runner: &dyn MainRunner = &();
    runner.run();
}