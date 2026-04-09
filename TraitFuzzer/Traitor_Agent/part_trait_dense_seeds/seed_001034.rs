trait Main {
    fn run(self);
}

impl Main for () {
    fn run(self) {}
}

fn main() {
    ().run();
}