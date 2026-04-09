trait MainRunner {
    fn run(&self);
}

struct EmptyRunner;

impl MainRunner for EmptyRunner {
    fn run(&self) {}
}

fn main() {
    let runner: Box<dyn MainRunner> = Box::new(EmptyRunner);
    runner.run();
}