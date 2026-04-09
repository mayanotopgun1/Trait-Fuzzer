#![feature(try_as_dyn)]
use std::any::try_as_dyn;

trait Trait<T> {

}

impl Trait<for<'a> fn(&'a Box<i32>)> for () {

}

trait CheckTrait {
    fn check_trait(&self) -> bool;
}

impl dyn Trait<fn(&'static Box<i32>)> + 'static {
    fn check_trait(&self) -> bool {
        true
    }
}

fn main() {
    let dt = try_as_dyn::<_, dyn Trait<fn(&'static Box<i32>)>>(&());
    assert!(dt.is_none());

    if let Some(t) = dt {
        assert!(t.check_trait());
    }
}