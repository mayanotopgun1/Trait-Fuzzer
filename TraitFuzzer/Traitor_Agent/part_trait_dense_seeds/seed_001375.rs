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

fn make_grad_fn(f: impl Fn() -> usize) -> GradFn<impl Fn() -> usize> {
    GradFn(f)
}

fn main() {
    let x_squared = make_grad_fn(|| -> usize { 2 });
    let _ = x_squared.call();
}