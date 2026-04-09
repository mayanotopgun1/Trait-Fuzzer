#![allow(dead_code)]
#![feature(impl_trait_in_assoc_type)]

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
    let _: <FnType<T> as TheTrait>::TheType = unimplemented!();
    wf::<<FnType<T> as TheTrait>::TheType>();
}

fn main() {}