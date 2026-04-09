#![feature(trait_alias)]

#[no_mangle]
extern "C" fn foo() {}

trait Execute {
    fn execute(&self);
}

impl Execute for () {
    fn execute(&self) {
        foo()
    }
}

type DynExecute = dyn Execute;

fn main() {
    let executor: &DynExecute = &();
    executor.execute();
}