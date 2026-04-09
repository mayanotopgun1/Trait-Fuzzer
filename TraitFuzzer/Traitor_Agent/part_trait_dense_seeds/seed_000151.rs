#![feature(specialization)]
#![feature(type_alias_impl_trait)]

pub type Ty<'a> = impl Sized + 'a;
#[define_opaque(Ty)]
fn define<'a>() -> Ty<'a> {}

trait CallTy {
    fn call(&self, _: &'static fn(Ty<'_>));
}

impl<F: Fn(&'static fn(Ty<'_>))> CallTy for F {
    fn call(&self, f: &'static fn(Ty<'_>)) {
        self(f);
    }
}

fn test1(caller: impl CallTy) {}

trait NoneOp {
    fn none(&self) -> Option<&'static fn(Ty<'_>)>;
}

default impl<T> NoneOp for T {
    fn none(&self) -> Option<&'static fn(Ty<'_>)> {
        None
    }
}

impl NoneOp for () {
    fn none(&self) -> Option<&'static fn(Ty<'_>)> {
        None
    }
}

fn test2(none: impl NoneOp) {
    let _ = none.none();
}

fn main() {}