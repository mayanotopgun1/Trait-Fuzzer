#![feature(return_position_impl_trait_in_trait)]

trait Callable {
    fn call(&mut self) -> impl FnMut();
}

impl<F: FnMut()> Callable for F {
    fn call(&mut self) -> impl FnMut() {
        move || self()
    }
}

fn f<C: Callable>(mut c: C) {
    c.call()();
}

fn main() {
    let mut v: Vec<_> = vec![];
    f(|| v.push(0));
    assert_eq!(v, [0]);
}