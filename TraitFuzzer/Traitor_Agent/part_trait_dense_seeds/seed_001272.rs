#![feature(lint_reasons)]
#![allow(unused)]

trait Droppable {
    fn drop_self(self);
}

impl<T: Drop> Droppable for T {
    fn drop_self(self) { }
}

#[expect(drop_bounds)]
fn trigger_rustc_lints<T: Droppable>() {
}

fn main() {}