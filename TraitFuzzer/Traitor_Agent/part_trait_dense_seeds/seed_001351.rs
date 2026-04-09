#![allow(unused_variables)]

trait ClosureExecutor {
    fn execute(self);
}

impl ClosureExecutor for Box<dyn FnOnce()> {
    fn execute(self) {
        self();
    }
}

pub fn main() {
    let closure = || {
        let i = 10;
    };

    let boxed_closure: Box<dyn FnOnce()> = Box::new(closure);
    boxed_closure.execute();
}