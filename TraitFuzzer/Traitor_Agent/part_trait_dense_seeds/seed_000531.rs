#![feature(type_alias_impl_trait)]

struct Foo {
    x: i32,
}

trait MutAccess<'a> {
    type Out;
    fn get_mut(&'a mut self) -> Self::Out;
}

impl<'a> MutAccess<'a> for Foo {
    type Out = &'a mut i32;
    fn get_mut(&'a mut self) -> Self::Out {
        &mut self.x
    }
}

trait Modify: MutAccess<'static> {
    fn modify(&mut self, value: i32);
}

impl Modify for Foo {
    fn modify(&mut self, value: i32) {
        *self.get_mut() = value;
    }
}

pub fn main() {
    let mut foo = Foo { x: 42 };
    foo.modify(13);
    let y = foo;
    assert_eq!(y.x, 13);
}