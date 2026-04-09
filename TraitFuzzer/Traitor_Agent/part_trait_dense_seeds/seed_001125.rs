pub enum Foo {
    Bar(()),
}

trait FooConsts {
    const BAR1: Foo;
    const BAR2: Foo;
    const BAR3: Self;
    const BAR4: Self;
}

impl FooConsts for Foo {
    const BAR1: Foo = Foo::Bar(());
    const BAR2: Foo = Self::Bar(());
    const BAR3: Self = Foo::Bar(());
    const BAR4: Self = Self::Bar(());
}

fn main() {}