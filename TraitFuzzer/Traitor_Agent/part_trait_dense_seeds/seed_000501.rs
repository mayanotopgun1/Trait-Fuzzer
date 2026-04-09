#![feature(type_alias_impl_trait)]

fn main() {}

trait Debuggable { fn debug(&self); }
type NoReveal = impl std::fmt::Debug + Debuggable;

impl<T: std::fmt::Debug> Debuggable for T {
    fn debug(&self) { println!("{:?}", self); }
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