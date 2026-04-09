#![feature(specialization)]

use std::ops::Deref;

trait Trait: Deref<Target = [u8; 2]> {}

trait LengthExt: Deref<Target = [u8; 2]>
where
    Self: Sized,
{
    fn length(&self) -> usize;
}

impl<S> LengthExt for S where S: Trait {
    default fn length(&self) -> usize {
        self.deref().len()
    }
}

fn main() {}