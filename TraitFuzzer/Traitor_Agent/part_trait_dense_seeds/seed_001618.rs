#![feature(impl_trait_in_assoc_type)]

trait Foo {
    type Bar<'a>
    where
        Self: Sized;

    fn test(&self);
}

trait FooExt: Foo {
    fn extended_test(&self) {
        self.test();
    }
}

impl<T> FooExt for T where T: Foo {}

impl Foo for () {
    type Bar<'a> = ()
    where
        Self: Sized;

    fn test(&self) {}
}

fn test(x: &dyn FooExt) {
    x.extended_test();
}

fn main() {
    test(&());
}