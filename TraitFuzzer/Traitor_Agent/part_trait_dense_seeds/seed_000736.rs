trait Invoke {
    fn invoke(&self);
}

impl<F: Fn()> Invoke for F {
    fn invoke(&self) {
        self()
    }
}

fn main() {
    let x: Vec<Box<dyn Invoke>> = vec![Box::new(|| ())];
    x[0].invoke();
}