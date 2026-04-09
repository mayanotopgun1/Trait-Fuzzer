#![feature(return_position_impl_trait_in_trait)]

trait Boxable<'a> {
    fn box_me(self) -> impl std::fmt::Debug;
}

impl<'a, T: 'a + std::fmt::Debug> Boxable<'a> for T {
    fn box_me(self) -> impl std::fmt::Debug {
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
    let _x = 1.box_me();
    let lam_move = || {};
    lam_move.call();
}