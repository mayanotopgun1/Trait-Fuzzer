#![feature(fn_traits)]

trait Reusable {
    fn call(&self);
}

impl<F> Reusable for F
where
    F: Fn(),
{
    fn call(&self) {
        self()
    }
}

fn main() {
    let closure = || println!("Hello, world!");
    let reusable_closure: Box<dyn Reusable> = Box::new(closure);

    reusable_closure.call();
}