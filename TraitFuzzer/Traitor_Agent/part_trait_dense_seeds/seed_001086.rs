#![feature(generic_associated_types)]

trait Trait {
    type Assoc<'a, 'b> where Self: 'a + 'b;
    const ASSOC: for<'a, 'b> fn(&'a u32, &'b u32) -> Self::Assoc<'a, 'b>;
}

trait TraitExt: Trait {
    const DOUBLE_ASSOC: for<'a, 'b> fn(&'a u32, &'b u32) -> Self::Assoc<'a, 'b>
        = |x, y| (Self::ASSOC)(x, y);
}

impl<T: Trait> TraitExt for T {}

impl Trait for () {
    type Assoc<'a, 'b> where Self: 'a + 'b = ();
    const ASSOC: for<'a, 'b> fn(&'a u32, &'b u32) -> Self::Assoc<'a, 'b>
        = |_, _| ();
}

fn main() {
    let _ = <() as TraitExt>::DOUBLE_ASSOC;
}