#![feature(generic_associated_types)]

trait Trait {
    type Item<'a>: 'a;
}

trait AssertStatic: 'static {}
impl<T: 'static> AssertStatic for T {}

fn assert_static<T: AssertStatic>(_: T) {}

trait TestArgsTrait<I: for<'a> Trait<Item<'a> = &'static ()>> {
    fn test_args(self);
}
impl<I: for<'a> Trait<Item<'a> = &'static ()>> TestArgsTrait<I> for () {
    fn test_args(self) {
        let closure = |a, _b| assert_static(a);

        closure(None::<I::Item<'_>>, &None::<I::Item<'_>>);
    }
}

trait TestUpvarsTrait<I: for<'a> Trait<Item<'a> = &'static ()>> {
    fn test_upvars(self);
}
impl<I: for<'a> Trait<Item<'a> = &'static ()>> TestUpvarsTrait<I> for () {
    fn test_upvars(self) {
        let upvars = (None::<I::Item<'_>>, &None::<I::Item<'_>>);
        let _closure = || {
            let (a, _b) = upvars;
            assert_static(a);
        };
    }
}

fn main() {}