#![feature(trait_alias)]

trait Bar<'a> {}

trait Foo<'a> = Bar<'a>;

trait Baz<'a>: Foo<'a> {}

impl<'a, T: Foo<'a>> Baz<'a> for T {}

fn test2(_: &(impl for<'a> Baz<'a> + ?Sized)) {}

fn test(x: &dyn for<'a> Baz<'a>) {
    test2(x);
}

fn main() {}