#![feature(type_alias_impl_trait)]
#![feature(trait_alias)]

struct Bar;
trait Foo {}
impl Foo for Bar {}

trait Baz = Foo where Self: Foo;

type HiddenBaz = Box<dyn Foo>;

fn new() -> HiddenBaz {
    Box::new(Bar)
}

trait NewExt { fn create() -> Self; }
impl NewExt for HiddenBaz { fn create() -> Self { new() } }

fn main() {
    let _ = <HiddenBaz as NewExt>::create();
}