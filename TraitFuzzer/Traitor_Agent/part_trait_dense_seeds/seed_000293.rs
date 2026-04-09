trait MainTrait {
    fn main(&self);
}

impl MainTrait for () {
    fn main(&self) {}
}

fn main() {
    let _ = Box::new(()).main();
}