#![feature(trait_alias)]

trait Bar<'a> {}

trait Foo<'a> = Bar<'a>;

trait Baz<'a>: Foo<'a> {
    fn qux(&self) -> Box<dyn core::fmt::Debug>;
}

impl<'a, T: Foo<'a>> Baz<'a> for T {
    fn qux(&self) -> Box<dyn core::fmt::Debug> { Box::new(42) }
}

fn test2(_: &(impl for<'a> Baz<'a> + ?Sized)) {}

fn test(x: &dyn for<'a> Baz<'a>) {
    test2(x);
    println!("{:?}", x.qux());
}

fn main() {}