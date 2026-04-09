#![feature(return_position_impl_trait_in_trait)]

#[derive(Debug)]
pub enum Foo {
    Bar(()),
}

trait FooConsts {
    const BAR1: Foo;
    const BAR2: Foo;
    const BAR3: Self;
    const BAR4: Self;

    fn foo_method(&self) -> impl core::fmt::Debug;
}

impl FooConsts for Foo {
    const BAR1: Foo = Foo::Bar(());
    const BAR2: Foo = Self::Bar(());
    const BAR3: Self = Foo::Bar(());
    const BAR4: Self = Foo::Bar(());

    fn foo_method(&self) -> impl core::fmt::Debug {
        self
    }
}

fn main() {}