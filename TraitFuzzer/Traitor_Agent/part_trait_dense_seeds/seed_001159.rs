use std::ops::Deref;

trait Trait: Deref<Target = [u8; { 1 + 1 }]> {}

trait LengthExt: Deref<Target = [u8; { 1 + 1 }]>
where
    Self: Sized,
{
    fn length(&self) -> usize {
        self.deref().len()
    }
}

impl<S> LengthExt for S where S: Trait {}

fn main() {}