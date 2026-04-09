#![feature(type_alias_impl_trait)]
#![allow(dead_code)]

pub(crate) type Foo = impl std::fmt::Debug + Opaque;

trait Opaque {
    fn get(self) -> u32;
}

#[derive(Debug)] // Add derive(Debug) to implement the Debug trait for FooImpl
struct FooImpl(u32);

impl Opaque for FooImpl {
    fn get(self) -> u32 {
        self.0
    }
}

#[define_opaque(Foo)]
fn foo() -> Foo {
    FooImpl(22_u32)
}

fn is_send<T: Send>(_: T) {}

fn main() {
    let foo_value = foo();
    is_send(foo_value.get());
}