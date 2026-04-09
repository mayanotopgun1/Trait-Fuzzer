trait Main {
    fn run(&self);
}

impl Main for () {
    fn run(&self) {}
}

fn get_main() -> impl Main {
    ()
}

fn main() {
    let _: Box<dyn Main> = Box::new(get_main());
    get_main().run();
}