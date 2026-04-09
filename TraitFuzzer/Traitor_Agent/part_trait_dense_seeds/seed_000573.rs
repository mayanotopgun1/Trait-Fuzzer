#![feature(return_position_impl_trait_in_trait)]
#![allow(dead_code)]

trait A {
    fn hello(&self) -> impl core::fmt::Debug {
        println!("A");
        "A"
    }
}
impl<T> A for T {}

trait B: A {
    fn hello(&self) -> impl core::fmt::Debug {
        println!("B");
        "B"
    }
}
impl<T> B for T {}

trait HelloTrait {
    fn call_hello(&self);
}

trait ExtendedHelloTrait: HelloTrait {
    fn extended_hello(&self) where Self: A {
        self.hello();
    }
}

impl<T: B> ExtendedHelloTrait for T {}
impl<T: ExtendedHelloTrait> HelloTrait for T {
    fn call_hello(&self) {
        self.extended_hello();
    }
}

fn foo<T: HelloTrait>(t: &T) {
    t.call_hello();
}

fn main() {
    foo::<()>(&());
}