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

fn f<'a>(x: &dyn ObjExt<'a, U = i32, V = i32>) {
    is_obj(x);
    x.debug();
}

fn main() {}