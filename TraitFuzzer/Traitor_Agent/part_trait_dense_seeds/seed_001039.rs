#![crate_type = "rlib"]
#![feature(type_alias_impl_trait)]

trait Foo<'a> { type Out; fn foo(&self) -> Self::Out; }

trait DoubleFoo<'a, T>: std::ops::Add<Output = T> + Copy {}

trait FooTwice<'a>: Foo<'a> where Self::Out: DoubleFoo<'a, Self::Out> {
    fn foo_twice(&self) -> Self::Out {
        let x = self.foo();
        x + x
    }
}

impl<'a, T> FooTwice<'a> for T where T: Foo<'a>, T::Out: DoubleFoo<'a, T::Out> {}

impl<'a> Foo<'a> for () {
    type Out = ();
    fn foo(&self) -> Self::Out { () }
}

#[no_mangle]
pub extern "C" fn foo() {
    let _x: () = ();
    let _y = _x.foo();
}