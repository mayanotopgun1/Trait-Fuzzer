#![feature(specialization)]

use std::cell::Cell;
use std::ops::{Deref, DerefMut};

trait Countable<T> {
    fn new(value: T) -> Self;
    fn counts(&self) -> (usize, usize);
}

default impl<T> Countable<T> for T {
    fn new(value: T) -> T {
        value
    }

    fn counts(&self) -> (usize, usize) {
        (0, 0)
    }
}

impl<T> Countable<T> for DerefCounter<T> {
    fn new(value: T) -> DerefCounter<T> {
        DerefCounter {
            count_imm: Cell::new(0),
            count_mut: 0,
            value
        }
    }

    fn counts(&self) -> (usize, usize) {
        (self.count_imm.get(), self.count_mut)
    }
}

trait Accessible {
    fn get(&self) -> (isize, isize);
}

default impl<T> Accessible for T {
    fn get(&self) -> (isize, isize) {
        (0, 0)
    }
}

impl<T> Accessible for DerefCounter<T>
where
    T: Accessible,
{
    fn get(&self) -> (isize, isize) {
        self.value.get()
    }
}

#[derive(PartialEq)]
struct DerefCounter<T> {
    count_imm: Cell<usize>,
    count_mut: usize,
    value: T
}

impl<T> Deref for DerefCounter<T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.count_imm.set(self.count_imm.get() + 1);
        &self.value
    }
}

impl<T> DerefMut for DerefCounter<T> {
    fn deref_mut(&mut self) -> &mut T {
        self.count_mut += 1;
        &mut self.value
    }
}

#[derive(PartialEq, Debug)]
struct Point {
    x: isize,
    y: isize
}

impl Accessible for Point {
    fn get(&self) -> (isize, isize) {
        (self.x, self.y)
    }
}

pub fn main() {
    let mut p: DerefCounter<Point> = Countable::new(Point {x: 0, y: 0});

    let _ = p.x;
    assert_eq!(p.counts(), (1, 0));

    let _ = &p.x;
    assert_eq!(p.counts(), (2, 0));

    let _ = &mut p.y;
    assert_eq!(p.counts(), (2, 1));

    p.x += 3;
    assert_eq!(p.counts(), (2, 2));

    Accessible::get(&p);
    assert_eq!(p.counts(), (3, 2));

    assert_eq!(*p, Point {x: 3, y: 0});
}