struct Foo;

trait Recur {
    fn recur(&self, b: bool) -> &Self;
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