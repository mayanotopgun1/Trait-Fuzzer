#![feature(type_alias_impl_trait, rustc_attrs)]
#![rustc_no_implicit_bounds]

trait Foo {
    #[cfg(normalize_param_env)]
    type Gat<'a>: Bar where <Self as Mirror>::Assoc: 'a;
    #[cfg(normalize_obligation)]
    type Gat<'a>: Bar where Self: 'a;
    #[cfg(hrtb)]
    type Gat<'b>: Bar where for<'a> <Self as MirrorRegion<'a>>::Assoc: 'b;
}

trait Mirror { type Assoc; }
impl<T> Mirror for T { type Assoc = T; }

trait MirrorRegion<'a> { type Assoc; }
impl<'a, T> MirrorRegion<'a> for T { type Assoc = T; }

trait FooImpl {
    #[cfg(normalize_param_env)]
    fn gat<'a>(&self) -> Self::Gat<'a> where <Self as Mirror>::Assoc: 'a;
    #[cfg(normalize_obligation)]
    fn gat<'a>(&self) -> Self::Gat<'a> where Self: 'a;
    #[cfg(hrtb)]
    fn gat<'b>(&self) -> Self::Gat<'b> where for<'a> <Self as MirrorRegion<'a>>::Assoc: 'b;
}

impl<T> FooImpl for T {
    #[cfg(normalize_param_env)]
    fn gat<'a>(&self) -> Self::Gat<'a> where <Self as Mirror>::Assoc: 'a { 0 }
    #[cfg(normalize_obligation)]
    fn gat<'a>(&self) -> Self::Gat<'a> where Self: 'a { 0 }
    #[cfg(hrtb)]
    fn gat<'b>(&self) -> Self::Gat<'b> where for<'a> <Self as MirrorRegion<'a>>::Assoc: 'b { 0 }
}

impl<T> Foo for T {
    #[cfg(normalize_param_env)]
    type Gat<'a> = impl Bar where T: 'a;
    #[cfg(normalize_obligation)]
    type Gat<'a> = impl Bar where <T as Mirror>::Assoc: 'a;
    #[cfg(hrtb)]
    type Gat<'b> = impl Bar where Self: 'b;
}

trait Bar {}

impl Bar for i32 {}

fn main() {}