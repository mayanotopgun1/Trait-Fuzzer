#![feature(cfg_target_compact)]

trait ConditionalExecution {
    fn execute(&self);
}

struct Expected;
impl ConditionalExecution for Expected {
    fn execute(&self) {
        #[cfg(target(os = "linux", arch = "arm"))]
        expected();
    }
}

struct Unexpected;
impl ConditionalExecution for Unexpected {
    fn execute(&self) {
        #[cfg(target(os = "linux", pointer_width = "X"))]
        unexpected();
    }
}

fn main() {
    let _expected = Expected;
    let _unexpected = Unexpected;

    _expected.execute();
    _unexpected.execute();
}