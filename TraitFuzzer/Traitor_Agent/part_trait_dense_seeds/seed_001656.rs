#![feature(type_alias_impl_trait)]
#![allow(non_shorthand_field_patterns)]

struct T { a: Box<isize> }

trait U {
    fn f(self);
}

impl U for Box<isize> {
    fn f(self) { }
}

type HiddenU = impl U;

#[define_opaque(HiddenU)]
fn get_hidden_u() -> HiddenU {
    Box::new(0)
}

trait V: U {}

impl<T: U> V for T {}

pub fn main() {
    let a = get_hidden_u();
    a.f();
}