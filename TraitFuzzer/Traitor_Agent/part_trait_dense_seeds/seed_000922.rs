#![feature(specialization)]

trait Is {
    type T;
}

impl<U> Is for U {
    type T = U;
}

trait Obj<'a> {
    type U: Is<T = Self::V>;
    type V;
}

trait ObjExt<'a>: Obj<'a> {}
impl<'a, T: ?Sized + Obj<'a>> ObjExt<'a> for T {}

fn is_obj<'a, T: ?Sized + Obj<'a>>(_: &T) {}

trait ObjDebug<'a>: Obj<'a> {
    fn debug(&self);
}

impl<'a, T: ?Sized + Obj<'a>> ObjDebug<'a> for T {
    default fn debug(&self) {}
}

fn f<'a>(x: &dyn ObjExt<'a, U = i32, V = i32>) -> impl core::fmt::Debug {
    is_obj(x);
    x.debug();
    42
}

struct Example;

impl<'a> Obj<'a> for Example {
    type U = i32;
    type V = i32;
}

impl<'a> ObjExt<'a> for Example {}

impl<'a> ObjDebug<'a> for Example {}

fn main() { let _ = f(&Example); }