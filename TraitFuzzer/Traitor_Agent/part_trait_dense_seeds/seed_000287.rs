trait PanicOps {
    fn bigpanic(&self);
}

impl PanicOps for () {
    fn bigpanic(&self) {
        while panic!("oops") {}
    }
}

fn main() {
    let unit = Box::new(());
    unit.bigpanic();
}