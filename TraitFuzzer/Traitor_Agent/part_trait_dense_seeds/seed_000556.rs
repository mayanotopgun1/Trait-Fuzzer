#![feature(return_position_impl_trait_in_trait)]

trait Foo {
    fn f(&self) -> impl core::fmt::Debug;
}

struct Bar {
    x: isize
}

trait Baz {
    fn g(&self);
}

impl<T:Baz> Foo for T {
    fn f(&self) -> impl core::fmt::Debug {
        self.g();
        0
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
    fn enhanced_f(&self) -> impl core::fmt::Debug {
        self.f();
        1
    }
}

impl<T:Foo> FooExt for T {}

pub fn main() {
    let y = Bar { x: 42 };
    println!("{:?}", y.enhanced_f());
}