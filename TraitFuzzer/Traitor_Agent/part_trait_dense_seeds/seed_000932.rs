#![feature(trait_alias)]
#![feature(impl_trait_in_assoc_type)]

struct Bar;
trait Foo {}
impl Foo for Bar {}

trait Baz = Foo where Bar: Foo;

trait BazExt: Baz {}
impl<T> BazExt for T where T: Baz {}

fn new() -> impl Baz {
    Bar
}

fn main() {
    let _ = new();
}