#![feature(type_alias_impl_trait)]

#[derive(Copy, Clone)]
struct Foo((u32, u32));

trait Copyable { }
impl<T: Copy> Copyable for T {}

fn main() {
    type T = impl Copy + Copyable;
    let foo: T = Foo((1u32, 2u32));
    let x = move || {
        let Foo((a, b)) = foo;
    };
}