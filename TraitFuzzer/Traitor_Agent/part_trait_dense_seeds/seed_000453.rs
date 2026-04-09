#![feature(return_position_impl_trait_in_trait)]
#![allow(dead_code)]

trait Opaque {
    fn get(self) -> u32;
}

#[derive(Debug)]
struct FooImpl(u32);

impl Opaque for FooImpl {
    fn get(self) -> u32 {
        self.0
    }
}

trait FooGenerator {
    fn generate(&self) -> impl Opaque;
}

struct FooGen;

impl FooGenerator for FooGen {
    fn generate(&self) -> impl Opaque {
        FooImpl(22_u32)
    }
}

fn is_send<T: Send>(_: T) {}

fn main() {
    let foo_gen = FooGen;
    let foo_value = foo_gen.generate();
    is_send(foo_value.get());
}