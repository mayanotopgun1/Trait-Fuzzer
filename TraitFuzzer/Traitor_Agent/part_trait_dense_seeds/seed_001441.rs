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
    fn extended_apply(&self) where Self: Sized {
        self.apply::<Self>();
    }
}

impl<T> TraitETraitDExt for T where T: TraitE + for<'a> TraitD<Scalar = i32>, T: Sized {}

fn main() {}