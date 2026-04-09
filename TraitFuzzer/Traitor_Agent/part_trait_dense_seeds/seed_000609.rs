#![feature(impl_trait_in_bindings)]

trait Identity { fn id(self) -> Self; }
impl Identity for () { fn id(self) -> Self { self } }

fn f(u: ()) -> impl Identity {
    u.id()
}

pub fn main() {
    let u1: () = ();
    let mut _u2: impl Identity = f(u1);
    _u2 = f(());
}