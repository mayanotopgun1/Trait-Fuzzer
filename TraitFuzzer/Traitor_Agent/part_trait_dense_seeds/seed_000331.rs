#![allow(nonstandard_style)]

pub mod bar {
    pub struct Foo { pub bar: Bar }
    pub struct Bar(pub char);

    pub trait FooTrait {
        fn new(bar: Bar) -> Self;
    }

    impl FooTrait for Foo {
        fn new(bar: Bar) -> Self {
            Foo { bar }
        }
    }
}

pub mod x {
    use crate::bar;
    pub const Foo: bar::Bar = bar::Bar('a');

    pub trait XTrait {
        const Foo: bar::Bar;
    }

    impl XTrait for () {
        const Foo: bar::Bar = bar::Bar('a');
    }
}

pub fn warning() -> bar::Foo {
    #![deny(unused_imports)]
    use bar::{FooTrait};
    use x::{XTrait};

    FooTrait::new(<() as XTrait>::Foo)
}

fn main() {}