#![allow(dead_code)]

trait TheTrait {
    type TheType;
}

trait TheTraitExt: TheTrait {}

impl<T: TheTrait> TheTraitExt for T {}

fn wf<T>() {}

type FnType<T> = for<'r> fn(&'r T);

fn foo<'a, 'b, T>()
where
    FnType<T>: TheTrait,
{
    wf::<<FnType<T> as TheTrait>::TheType>();
}

fn main() {}