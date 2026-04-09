#![feature(return_position_impl_trait_in_trait)]

trait A {
    fn foo(&self) -> impl core::fmt::Debug {}
}

trait B: A {
    fn bar(&self) -> impl core::fmt::Debug {}
}

impl A for i32 {
    fn foo(&self) -> impl core::fmt::Debug { *self }
}
impl A for f64 {
    fn foo(&self) -> impl core::fmt::Debug { *self }
}
impl B for f64 {
    fn bar(&self) -> impl core::fmt::Debug { *self }
}

trait AFoo {
    fn call_foo(&self);
}

impl<T> AFoo for T where T: A {
    fn call_foo(&self) {
        println!("{:?}", self.foo());
    }
}

trait BBar {
    fn call_bar(&self);
}

impl<T> BBar for T where T: B {
    fn call_bar(&self) {
        println!("{:?}", self.bar());
    }
}

fn main() {
    let x = 42i32;
    x.call_foo();

    let y = 3.14f64;
    y.call_bar();
}