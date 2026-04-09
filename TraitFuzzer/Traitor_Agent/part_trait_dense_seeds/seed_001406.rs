#![feature(type_alias_impl_trait)]

#[derive(Copy, Clone, Debug)]
pub enum Foo {
    Bar(()),
}

trait FooConstants {
    type FooType;
    const BAR1: Self::FooType;
    const BAR2: Self::FooType;
    const BAR3: Self::FooType;
    const BAR4: Self::FooType;
}

impl FooConstants for Foo {
    type FooType = Foo;

    const BAR1: Self::FooType = Foo::Bar(());
    const BAR2: Self::FooType = Foo::Bar(());
    const BAR3: Self::FooType = Foo::Bar(());
    const BAR4: Self::FooType = Foo::Bar(());
}

trait FooConstantsExt: FooConstants {
    fn all_bars(&self) -> [Self::FooType; 4] {
        [Self::BAR1, Self::BAR2, Self::BAR3, Self::BAR4]
    }
}

impl<T> FooConstantsExt for T where T: FooConstants {}

fn main() {}