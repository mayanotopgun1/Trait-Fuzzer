#![feature(type_alias_impl_trait, specialization)]

fn main() {}

trait Debuggable { fn debug(&self); }
type NoReveal = impl std::fmt::Debug + Debuggable;

default impl<T> Debuggable for T {
    default fn debug(&self) {}
}

impl Debuggable for &str {
    fn debug(&self) { println!("String: {:?}", self); }
}

#[define_opaque(NoReveal)]
fn define_no_reveal() -> NoReveal {
    ""
}

#[define_opaque(NoReveal)]
fn no_reveal(x: NoReveal) {
    x.debug();
    let _: &'static str = x;
    let _ = x as &'static str;
}