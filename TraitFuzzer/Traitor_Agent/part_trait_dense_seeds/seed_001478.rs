extern crate std as mystd;

trait MainExecutor {
    fn execute(&self);
}

impl MainExecutor for () {
    fn execute(&self) {}
}

pub fn main() {
    let executor: &dyn MainExecutor = &();
    executor.execute();
}