#![feature(generic_associated_types)]

trait Boxable<'a> {
    type Out;
    fn box_me(self) -> Self::Out;
}

impl<'a, T> Boxable<'a> for T {
    type Out = Box<T>;
    fn box_me(self) -> Self::Out {
        Box::new(self)
    }
}

trait Callable {
    fn call(self);
}

impl<F: FnOnce()> Callable for F {
    fn call(self) {
        self()
    }
}

pub fn main() {
    let _x: Box<i32> = 1.box_me();
    let lam_move = || {};
    lam_move.call();
}