#![feature(return_position_impl_trait_in_trait)]

trait MainLike {
    fn run(&self);
}

trait MainExt: MainLike where Self: Sized {
    fn double_run(&self) -> impl FnOnce();
}

impl<T: MainLike + Sized> MainExt for T {
    fn double_run(&self) -> impl FnOnce() {
        move || { self.run(); self.run(); }
    }
}

impl MainLike for () {
    fn run(&self) {}
}

fn main() {
    let runner = ().double_run();
    runner();
}