#![feature(min_specialization)]

trait Foo {
    fn bar();
}

trait FooExt: Foo {
    fn baz(&self) where Self: Sized {
        Self::bar()
    }
}

impl<T> FooExt for T where T: Foo {}

impl<T> Foo for T {
    default fn bar() {}
}

impl Foo for () {
    fn bar() {}
}

fn main() {
    <() as FooExt>::baz(&());
}