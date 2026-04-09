#![feature(type_alias_impl_trait)]

trait B {
    type C;
}

struct A;

impl<'a> B for &'a A {
    type C = ();
}

struct Terminator;

type Successors<'a> = impl std::fmt::Debug + 'a;

trait SuccessorExt: Sized {
    fn successors_ext(&self, f: for<'x> fn(&'x ()) -> <&'x A as B>::C) -> Successors<'_>;
}

impl SuccessorExt for Terminator {
    #[define_opaque(Successors)]
    fn successors_ext(&self, _: for<'x> fn(&'x ()) -> <&'x A as B>::C) -> Successors<'_> {}
}

trait OriginalSuccessors {
    fn successors(&self, f: for<'x> fn(&'x ()) -> <&'x A as B>::C) -> Successors<'_>;
}

impl OriginalSuccessors for Terminator {
    #[define_opaque(Successors)]
    fn successors(&self, _: for<'x> fn(&'x ()) -> <&'x A as B>::C) -> Successors<'_> {}
}

fn main() {}