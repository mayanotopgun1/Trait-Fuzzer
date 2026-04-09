#![feature(specialization)]

use std::cell::Cell;

trait DropHandler<'a> {
    fn drop(&mut self);
}

default impl<'a, T> DropHandler<'a> for T {
    default fn drop(&mut self) {}
}

struct Defer<'a> {
    b: &'a Cell<bool>,
}

impl<'a> DropHandler<'a> for Defer<'a> {
    fn drop(&mut self) {
        self.b.set(true);
    }
}

fn defer(b: &Cell<bool>) -> impl DropHandler<'_> {
    Defer { b }
}

pub fn main() {
    let dtor_ran = Cell::new(false);
    let _defer = defer(&dtor_ran);
    assert!(dtor_ran.get());
}