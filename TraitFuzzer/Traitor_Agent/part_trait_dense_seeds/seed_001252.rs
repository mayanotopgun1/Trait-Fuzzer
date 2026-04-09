#![feature(supertrait_item_shadowing)]
#![allow(dead_code)]

trait A {
    fn hello() {
        println!("A");
    }
}
impl<T> A for T {}

trait B: A {
    fn hello() {
        println!("B");
    }
}
impl<T> B for T {}

trait C: B {
    fn hello_c() {
        println!("C");
    }
}
impl<T: B> C for T {}

fn foo<T>() where T: A {
    T::hello();
}

fn main() {
    foo::<()>();
}