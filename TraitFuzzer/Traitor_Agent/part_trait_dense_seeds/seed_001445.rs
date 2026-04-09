#![feature(generic_associated_types)]

struct Foo {
    new: isize,
}

trait NewAccess<'a> {
    type Output;
    fn get_new(&'a self) -> Self::Output;
}

impl<'a> NewAccess<'a> for Foo {
    type Output = &'a isize;
    fn get_new(&'a self) -> Self::Output {
        &self.new
    }
}

trait NewTrait<'a>: NewAccess<'a> {
    fn double_get_new(&'a self) -> Self::Output {
        let v = self.get_new();
        v
    }
}

impl<'a, T: NewAccess<'a>> NewTrait<'a> for T {}

pub fn main() {
    let foo = Foo { new: 3 };
    assert_eq!(*foo.double_get_new(), 3);
}