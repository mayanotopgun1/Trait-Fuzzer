#![feature(frontmatter)]

trait MainRunner {
    fn run(&self);
}

impl MainRunner for () {
    fn run(&self) {}
}

fn main() {
    let runner: Box<dyn MainRunner> = Box::new(());
    runner.run();
}