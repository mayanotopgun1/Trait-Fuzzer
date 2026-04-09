#![feature(specialization)]

trait Invoke {
    fn invoke(&self);
}

default impl<T> Invoke for T {
    default fn invoke(&self) {}
}

impl<F: Fn()> Invoke for F {
    fn invoke(&self) {
        self()
    }
}

fn main() {
    let x: Vec<Box<dyn Invoke>> = vec![Box::new(|| println!("Hello, world!"))];
    x[0].invoke();
}