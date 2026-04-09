trait Trait {
    type Assoc<'a>;
}

trait TraitExt: Trait {
    fn process_assoc(&self, f: impl Fn(Self::Assoc<'_>)) {}
}

impl<T: Trait> TraitExt for T {}

struct Type;

impl Trait for Type {
    type Assoc<'a> = ();
}

fn main() {
    let t = Type;
    t.process_assoc(|_| ());
}