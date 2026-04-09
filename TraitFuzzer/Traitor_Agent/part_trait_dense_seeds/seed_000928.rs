#![feature(type_alias_impl_trait)]

trait Proj<'a> {
    type Assoc;
}

impl<'a, 'b, F: FnOnce() -> &'b ()> Proj<'a> for F {
    type Assoc = ();
}

trait ProjExt<'a>: Proj<'a> {
    fn use_proj(&self) {}
}

impl<'a, T: Proj<'a>> ProjExt<'a> for T {}

fn is_proj<F: for<'a> Proj<'a>>(f: F) {}

type HiddenProj<'a> = impl Sized + Proj<'a>;

#[define_opaque(HiddenProj)]
fn define<'a>() -> HiddenProj<'a> {
    || { &() }
}

trait UseHiddenProj<'a>: ProjExt<'a> {
    fn use_hidden_proj(&self) {}
}

impl<'a, T: ProjExt<'a>> UseHiddenProj<'a> for T {}

fn main() {}