#![feature(type_alias_impl_trait)]

trait TupleExt { fn extract(self) -> (u32, u32); }
impl TupleExt for (u32, u32) { fn extract(self) -> (u32, u32) { self } }

fn main() {
    type T = impl Copy + TupleExt;
    let foo: T = (1u32, 2u32);
    let (a, b) = foo.extract();
}