#![feature(type_alias_impl_trait)]

trait FooTrait: std::fmt::Debug {
    fn foo(b: bool) -> Self;
}

type Foo = u32;

impl FooTrait for u32 {
    fn foo(b: bool) -> Self {
        if b {
            return 42;
        }
        let x: u32 = FooTrait::foo(false);
        99
    }
}

fn bar(b: bool) -> impl std::fmt::Debug {
    if b {
        return 42;
    }
    let _x: u32 = bar(false);
    99
}

fn main() {}