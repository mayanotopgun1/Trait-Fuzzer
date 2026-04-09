#![feature(type_alias_impl_trait)]

type Foo = impl Sized;

struct Bar<T>(T);

trait BarTrait<T> {
    fn bar(self);
}

impl BarTrait<Foo> for Bar<Foo> {
    #[define_opaque(Foo)]
    fn bar(mut self) {
        self.0 = 42_u32;
    }
}

trait FooTrait {
    fn foo(self);
}

impl FooTrait for Bar<u32> {
    #[define_opaque(Foo)]
    fn foo(self) {
        let _ = Bar::<Foo>::bar(Bar(self.0));
    }
}

trait FooMaker {
    type Out;
    fn make() -> Self::Out;
}

impl FooMaker for () {
    type Out = Foo;

    #[define_opaque(Foo)]
    fn make() -> Foo {
        42_u32
    }
}

fn main() {}