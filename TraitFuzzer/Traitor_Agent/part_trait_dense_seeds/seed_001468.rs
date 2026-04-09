#![feature(impl_trait_in_assoc_type, generic_associated_types, decl_macro)]

trait IceMacro<'a> {
    type Out;
    fn ice(&'a self) -> Self::Out;
}

trait IceEcho<'a>: IceMacro<'a> {
    fn echo(&'a self) -> impl core::fmt::Debug where Self::Out: Copy + core::fmt::Debug { self.ice() }
}

impl<'a, T: IceMacro<'a>> IceEcho<'a> for T {}

impl<'a> IceMacro<'a> for () {
    type Out = &'a ();
    fn ice(&'a self) -> Self::Out { self }
}

fn main() {
    let _ = <()>::echo(&());
}