trait Trait {
    const ASSOC: for<'a, 'b> fn(&'a u32, &'b u32);
}

trait TraitExt: Trait {
    fn call_assoc(&self, a: &u32, b: &u32) {
        (Self::ASSOC)(a, b);
    }
}

impl<T: Trait> TraitExt for T {}

impl Trait for () {
    const ASSOC: for<'a> fn(&'a u32, &'a u32) = |_, _| ();
}

fn main() {
    let _ = <() as TraitExt>::call_assoc(&(), &0, &0);
}