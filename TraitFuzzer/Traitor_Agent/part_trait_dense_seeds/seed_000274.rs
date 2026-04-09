trait Trait {
    type Value;
}

impl<'a> Trait for &'a () {
    type Value = ();
}

trait TraitExt: Trait {
    fn transform(&self) -> Self::Value {
        unimplemented!()
    }
}

impl<T: Trait> TraitExt for T {}

type X<'a> = <&'a () as Trait>::Value;

fn f(_: X) -> X {
    unimplemented!()
}

fn g<'a>(_: X<'a>) -> X<'a> {
    unimplemented!()
}

trait Transformable: Trait {
    fn apply_transform(&self) -> Self::Value;
}

impl<T: TraitExt> Transformable for T {
    fn apply_transform(&self) -> Self::Value {
        self.transform()
    }
}

fn main() {}