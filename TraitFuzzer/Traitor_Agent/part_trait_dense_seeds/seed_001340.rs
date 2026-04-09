#![feature(type_alias_impl_trait, try_as_dyn)]
use std::any::try_as_dyn;

trait Trait<T> {}

impl Trait<for<'a> fn(&'a Box<i32>)> for () {}

type DynTraitAlias = dyn Trait<fn(&'static Box<i32>)> + 'static;

trait CheckTrait {
    fn check_trait(&self) -> bool;
}

impl CheckTrait for DynTraitAlias {
    fn check_trait(&self) -> bool {
        true
    }
}

fn main() {
    let dt: Option<&DynTraitAlias> = try_as_dyn::<_, DynTraitAlias>(&());
    assert!(dt.is_none());

    if let Some(t) = dt {
        assert!(t.check_trait());
    }
}