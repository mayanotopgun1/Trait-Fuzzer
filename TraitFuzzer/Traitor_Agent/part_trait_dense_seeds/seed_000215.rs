trait MainRunner {
    fn run(self);
}

impl MainRunner for () {
    fn run(self) {}
}

fn main() {
    ().run();
}