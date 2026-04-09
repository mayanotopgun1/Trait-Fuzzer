#![feature(fn_traits)]

struct GradFn<F: Fn() -> usize>(F);

trait Call {
    fn call(&self) -> usize;
}

impl<F: Fn() -> usize> Call for GradFn<F> {
    fn call(&self) -> usize {
        (self.0)()
    }
}

fn main() {
    let GradFn(x_squared) : GradFn<_> = GradFn(|| -> usize { 2 });
    let _  = x_squared.call(());
}