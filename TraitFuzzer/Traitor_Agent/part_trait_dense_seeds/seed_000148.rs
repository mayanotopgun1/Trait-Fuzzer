#![feature(register_tool)]
#![register_tool(known_tool)]

trait ToolRunner {
    fn run(&self);
}

impl ToolRunner for () {
    fn run(&self) {}
}

fn main() {
    let runner: &dyn ToolRunner = &();
    runner.run();
}