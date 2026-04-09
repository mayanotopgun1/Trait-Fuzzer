#![allow(dead_code)]

struct A<'a> {
    a: &'a i32,
    b: &'a i32,
}

trait AFoo<'a> {
    type Ref<'b>: 'b where Self: 'b;
    fn foo<'b>(&'b self) -> Self::Ref<'b>;
}

impl <'a> AFoo<'a> for A<'a> {
    type Ref<'b> = &'b Self where Self: 'b;
    fn foo<'b>(&'b self) -> Self::Ref<'b> {
        &self
    }
}

trait AFooView<'a>: AFoo<'a> {
    fn view_foo<'b>(&'b self) -> Self::Ref<'b> where Self::Ref<'b>: Copy {
        self.foo()
    }
}

impl<'a, T: AFoo<'a>> AFooView<'a> for T {}

fn main() { }