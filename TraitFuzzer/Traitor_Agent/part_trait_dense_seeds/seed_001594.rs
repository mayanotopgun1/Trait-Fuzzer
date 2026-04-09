trait MainRunner {
    fn run(&self);
}

struct Program;

impl MainRunner for Program {
    fn run(&self) {}
}

fn main() {
    let program = Program;
    let runner: &dyn MainRunner = &program;
    runner.run();
}