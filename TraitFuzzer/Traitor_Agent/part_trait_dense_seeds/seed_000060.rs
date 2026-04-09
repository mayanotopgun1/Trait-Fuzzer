#![feature(coroutines, coroutine_trait)]
#![feature(type_alias_impl_trait)]

trait Trait {}

impl<T> Trait for T {}

type Foo<'c> = impl Trait + 'c;

trait FooExt<'a>: Trait {
    type Output;
}

impl<'a, S: Trait> FooExt<'a> for S {
    type Output = Self;
}

trait FooHelper<'b>: Trait {
    fn help(&self) -> Self;
}

impl<'b, T: Trait> FooHelper<'b> for &T {
    fn help(&self) -> Self {
        self
    }
}

#[define_opaque(Foo)]
fn foo<'a>(rng: &'a ()) -> Foo<'a> {
    let result = rng.help();
    result
}

fn main() {}