#![feature(type_alias_impl_trait)]

trait FooTrait: Sized {
    fn create_pair() -> Self;
}

impl FooTrait for (i32, u8) {
    fn create_pair() -> Self {
        (42, 42)
    }
}

trait FooView: FooTrait {
    type Item: Copy;
    fn view(&self) -> Self::Item {
        self.get()
    }

    fn get(&self) -> Self::Item;
}

impl FooView for (i32, u8) {
    type Item = (i32, u8);

    fn get(&self) -> Self::Item {
        *self
    }
}

type Foo = impl FooTrait;

#[define_opaque(Foo)]
pub fn foo() -> Foo {
    <(i32, u8)>::create_pair()
}

fn main() {}