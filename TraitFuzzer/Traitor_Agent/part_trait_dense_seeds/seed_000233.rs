trait EmptyMain {
    fn empty_main(&self);
}

impl EmptyMain for () {
    fn empty_main(&self) {}
}

fn main() {
    let _: Box<dyn EmptyMain> = Box::new(());
    ().empty_main();
}