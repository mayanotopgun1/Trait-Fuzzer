trait PanicTrait {
    fn fail(&self);
}

impl PanicTrait for () {
    fn fail(&self) {
        panic!();
    }
}

fn main() {
    let _ = Box::new(0);
    ().fail();
}