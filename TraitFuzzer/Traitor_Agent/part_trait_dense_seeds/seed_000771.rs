#![allow(non_camel_case_types)]
use std::cell::Cell;

#[derive(Debug)]
struct r<'a> {
    i: &'a Cell<isize>,
}

trait DropIncrement: Drop {
    fn increment(&self);
}

impl<'a> Drop for r<'a> {
    fn drop(&mut self) {
        self.increment();
    }
}

impl<'a> DropIncrement for r<'a> {
    fn increment(&self) {
        self.i.set(self.i.get() + 1);
    }
}

fn r(i: &Cell<isize>) -> r<'_> {
    r { i }
}

pub fn main() {
    let i = &Cell::new(0);

    {
        let a = r(i);
        let b = (a, 10);
        let (c, _d) = b;
        println!("{:?}", c);
    }
    assert_eq!(i.get(), 1);
}