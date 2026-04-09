#![feature(type_alias_impl_trait)]

trait Upvar {
    fn upvar(&self);
}

impl<T: Copy> Upvar for T {
    fn upvar(&self) {}
}

fn enum_upvar() {
    type T = impl Upvar + Copy;
    let foo: T = Some((1u32, 2u32));
    let x = move || match foo {
        None => (),
        Some((a, b)) => (),
    };
}

fn main(){}