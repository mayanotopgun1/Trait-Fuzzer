trait EmptyMain {
    fn empty_main(&self);
}

impl EmptyMain for () {
    fn empty_main(&self) {}
}

fn main() {
    let _: () = ();
    ().empty_main();
}