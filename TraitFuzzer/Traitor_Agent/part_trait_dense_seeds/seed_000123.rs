#![feature(generic_associated_types)]
#![warn(unused_variables, unreachable_code)]

enum Foo {}

struct S;

trait SMethod {
    type Out<'a> where Self: 'a;
    fn f(&self) -> Self::Out<'_>;
}

trait SMethExt: SMethod {
    fn g(&self) -> Option<Self::Out<'_>> { Some(self.f()) }
}

impl<T: SMethod> SMethExt for T {}

impl SMethod for S {
    type Out<'a> = &'a Foo;
    fn f(&self) -> Self::Out<'_> { todo!() }
}

fn main() {
    let s = S;
    let x: &Foo = s.g().unwrap();

    let _y = x;
}