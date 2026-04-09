#![feature(rustc_attrs)]
#![rustc_no_implicit_bounds]

trait Foo {
    #[cfg(normalize_param_env)]
    type Gat<'a> where <Self as Mirror>::Assoc: 'a;
    #[cfg(normalize_obligation)]
    type Gat<'a> where Self: 'a;
    #[cfg(hrtb)]
    type Gat<'b> where for<'a> <Self as MirrorRegion<'a>>::Assoc: 'b;
}

trait Mirror { type Assoc; }
impl<T> Mirror for T { type Assoc = T; }

trait MirrorRegion<'a> { type Assoc; }
impl<'a, T> MirrorRegion<'a> for T { type Assoc = T; }

trait FooImpl {
    #[cfg(normalize_param_env)]
    fn gat<'a>(&self) -> i32 where <Self as Mirror>::Assoc: 'a;
    #[cfg(normalize_obligation)]
    fn gat<'a>(&self) -> i32 where Self: 'a;
    #[cfg(hrtb)]
    fn gat<'b>(&self) -> i32 where for<'a> <Self as MirrorRegion<'a>>::Assoc: 'b;
}

impl<T> FooImpl for T {
    #[cfg(normalize_param_env)]
    fn gat<'a>(&self) -> i32 where <Self as Mirror>::Assoc: 'a { 0 }
    #[cfg(normalize_obligation)]
    fn gat<'a>(&self) -> i32 where Self: 'a { 0 }
    #[cfg(hrtb)]
    fn gat<'b>(&self) -> i32 where for<'a> <Self as MirrorRegion<'a>>::Assoc: 'b { 0 }
}

impl<T> Foo for T {
    #[cfg(normalize_param_env)]
    type Gat<'a> = i32 where T: 'a;
    #[cfg(normalize_obligation)]
    type Gat<'a> = i32 where <T as Mirror>::Assoc: 'a;
    #[cfg(hrtb)]
    type Gat<'b> = i32 where Self: 'b;
}

fn main() {}