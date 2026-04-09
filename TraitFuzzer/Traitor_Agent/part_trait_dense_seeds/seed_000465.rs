#![feature(return_position_impl_trait_in_trait)]

struct Invariant<T>(*mut T);

trait Opaque<'a> {
    fn opaque(_: &'a str) -> impl Opaque<'a>;
}

impl<'a> Opaque<'a> for Invariant<()> {
    fn opaque(_: &'a str) -> impl Opaque<'a> {
        Invariant(&mut ())
    }
}

fn main() {
    let binding = String::new();
    let x = <Invariant<()> as Opaque>::opaque(&binding);
    drop(x);
}