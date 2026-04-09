#![feature(const_trait_impl)]

trait Callable {
    fn call();
}

impl Callable for () {
    fn call() {}
}

fn foo<F: FnOnce()>(a: F) {}

fn bar() -> impl Callable {
    ()
}

fn main() {
    let _ = <() as Callable>::call;
    foo(|| ());
}