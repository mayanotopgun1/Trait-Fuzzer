#![feature(const_trait_impl)]

trait Callable {
    fn call();
}

impl Callable for () {
    fn call() {}
}

fn foo<F: FnOnce()>(a: F) {}

fn main() {
    let _ = <() as Callable>::call;
    foo(|| ());
}