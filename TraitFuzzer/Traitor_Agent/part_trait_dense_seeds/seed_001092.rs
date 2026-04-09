trait Callable {
    fn call(&mut self);
}

impl<F: FnMut()> Callable for F {
    fn call(&mut self) {
        self()
    }
}

fn f<C: Callable>(mut c: C) {
    c.call();
}

fn main() {
    let mut v: Vec<_> = vec![];
    f(|| v.push(0));
    assert_eq!(v, [0]);
}