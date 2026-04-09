#![feature(supertrait_item_shadowing)]
#![allow(dead_code)]

struct W<T>(T);

trait Upstream {
    fn hello(&self) {}
}
impl<T> Upstream for T {}

trait Downstream: Upstream {
    fn hello(&self) {}
}
impl<T> Downstream for W<T> where T: Foo {}

trait Foo {}

trait ExtendedUpstream: Upstream {
    fn extended_hello(&self) { self.hello() }
}
impl<T: Upstream> ExtendedUpstream for T {}

fn main() {
    let x = W(1i32);
    x.extended_hello();
}