#![feature(specialization)]

struct Foo;

trait Recur {
    fn recur(&self, b: bool) -> &Self;
}

default impl<T> Recur for T {
    fn recur(&self, _b: bool) -> &Self {
        self
    }
}

impl Recur for Foo {
    fn recur(&self, b: bool) -> &Self {
        if b {
            let temp = Foo;
            temp.recur(false);
        }

        self
    }
}

trait InClosure {
    fn in_closure(&self) -> &Self;
}

default impl<T> InClosure for T {
    fn in_closure(&self) -> &Self {
        self
    }
}

impl InClosure for Foo {
    fn in_closure(&self) -> &Self {
        let _ = || {
            let temp = Foo;
            temp.in_closure();
        };

        self
    }
}

fn main() {}