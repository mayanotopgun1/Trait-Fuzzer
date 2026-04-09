pub trait MainTrait {
    fn run(&self);
}

impl MainTrait for () {
    fn run(&self) {}
}

pub fn main() {
    let unit: Box<dyn MainTrait> = Box::new(());
    unit.run();
}