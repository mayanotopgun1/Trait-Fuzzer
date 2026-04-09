#![feature(generic_associated_types)]

trait Droppable {
    type Target<'a> where Self: 'a;
    fn drop(&mut self);
}

impl Droppable for A {
    type Target<'a> = &'a A where Self: 'a;
    fn drop(&mut self) {}
}

#[derive(PartialEq)]
struct A { x: usize }

impl Drop for A {
    fn drop(&mut self) {
        Droppable::drop(self)
    }
}

trait DroppableExt: Droppable {
    fn custom_drop(&mut self) {
        Droppable::drop(self);
    }
}

impl<T> DroppableExt for T where T: Droppable {}

pub fn main() {}