#![feature(lazy_type_alias)]
#![allow(incomplete_features)]

type Ref<'a> = &'a ();

trait Map<'a> {
    fn map(&self, input: Ref<'a>) -> Ref<'a>;
}

impl<'a, F> Map<'a> for F
where
    F: for<'b> Fn(Ref<'b>) -> &'b (),
{
    fn map(&self, input: Ref<'a>) -> Ref<'a> {
        self(input)
    }
}

fn map0(_: Ref) -> Ref { &() }
fn map1(_: Ref<'_>) -> Ref<'_> { &() }

fn main() {}