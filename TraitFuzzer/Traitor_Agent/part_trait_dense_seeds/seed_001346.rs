trait MainFn {
    fn run();
}

impl MainFn for () {
    fn run() {}
}

fn main() {
    <() as MainFn>::run();
}