#![feature(trait_alias)]

trait Foo = PartialEq<i32> + Send;
trait Bar = Foo + Sync;

trait I32Iterator = Iterator<Item = i32>;

trait EqCheck<T> {
    fn eq_check(&self, other: &T) -> bool;
}

impl<T: ?Sized + PartialEq<U>, U> EqCheck<U> for T {
    fn eq_check(&self, other: &U) -> bool {
        self == other
    }
}

pub fn main() {
    let a: &dyn Bar = &123;
    assert!(a.eq_check(&123));
    let b = Box::new(456) as Box<dyn Foo>;
    assert!(b.as_ref().eq_check(&456));

    let c: &mut dyn I32Iterator = &mut vec![123].into_iter();
    assert_eq!(c.next(), Some(123));
}