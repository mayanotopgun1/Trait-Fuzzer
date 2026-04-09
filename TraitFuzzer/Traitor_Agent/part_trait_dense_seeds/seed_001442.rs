#![feature(specialization)]

trait TraitB<T> {}

trait TraitC: TraitB<Self::Value> {
    type Value;
}

trait TraitD: TraitC<Value = Self::Scalar> {
    type Scalar;
}

trait TraitE {
    fn apply<PF: TraitD<Scalar = i32>>(&self);
}

trait TraitETraitDExt: TraitE + for<'a> TraitD<Scalar = i32> {
    fn extended_apply(&self) where Self: Sized;
}

impl<T> TraitETraitDExt for T where T: TraitE + for<'a> TraitD<Scalar = i32>, T: Sized {
    default fn extended_apply(&self) {
        self.apply::<Self>();
    }
}

fn main() {}