#![deny(warnings)]

trait Callable {
    fn call(self);
}

impl<F> Callable for F
where
    F: FnOnce(),
{
    fn call(self) {
        self()
    }
}

fn foo<C: Callable>(c: C) {
    c.call();
}

fn main() {
    let mut var = Vec::new();
    foo(move || {
        var.push(1);
    });
}