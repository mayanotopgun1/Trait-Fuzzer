#[deprecated]
trait MainTrait {
    fn run(&self);
}

impl MainTrait for () {
    fn run(&self) {}
}

fn main() {
    let _ = (()).run();
}