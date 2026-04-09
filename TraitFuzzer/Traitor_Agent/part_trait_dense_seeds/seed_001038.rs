#![crate_type = "rlib"]

trait Foo<'a> { type Out; fn foo(&self) -> Self::Out; }

trait FooTwice<'a>: Foo<'a> where Self::Out: std::ops::Add<Output = Self::Out> + Copy {
    fn foo_twice(&self) -> Self::Out {
        let x = self.foo();
        x + x
    }
}

impl<'a, T> FooTwice<'a> for T where T: Foo<'a>, T::Out: std::ops::Add<Output = T::Out> + Copy {}

impl<'a> Foo<'a> for () {
    type Out = ();
    fn foo(&self) -> Self::Out { () }
}

#[no_mangle]
pub extern "C" fn foo() {
    let _x: () = ();
    let _y = _x.foo();
}