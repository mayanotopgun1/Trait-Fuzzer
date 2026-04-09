#![feature(type_alias_impl_trait)]

trait Closer {
    fn call(self);
}

impl<F> Closer for F
where
    F: FnOnce(),
{
    fn call(self) {
        self();
    }
}

pub type Closure = impl Closer;

#[define_opaque(Closure)]
fn bop() -> Closure {
    || -> Closure { || () };
    panic!()
}

fn main() {}