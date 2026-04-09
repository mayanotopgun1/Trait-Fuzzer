#![feature(type_alias_impl_trait)]

trait Foo {
    fn f(&self) -> ();
}

struct Bar {
    x: isize
}

trait Baz {
    fn g(&self);
}

impl<T:Baz> Foo for T {
    fn f(&self) -> () {
        self.g();
    }
}

impl Baz for Bar {
    fn g(&self) {
        println!("{}", self.x);
    }
}

trait BazExt: Baz {
    fn double_g(&self) -> impl core::fmt::Debug {
        self.g();
        self.g();
        84
    }
}

impl<T:Baz> BazExt for T {}

trait FooExt: Foo {
    fn enhanced_f(&self) -> () {
        self.f();
    }
}

impl<T:Foo> FooExt for T {}

pub fn main() {
    let y = Bar { x: 42 };
    y.enhanced_f();
}