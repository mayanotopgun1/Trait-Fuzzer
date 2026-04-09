trait A {
    fn foo(&self) {}
}

trait B: A {
    fn bar(&self) {}
}

impl A for i32 {}
impl A for f64 {}
impl B for f64 {}

trait AFoo {
    fn call_foo(&self);
}

impl<T> AFoo for T where T: A {
    fn call_foo(&self) {
        self.foo()
    }
}

trait BBar {
    fn call_bar(&self);
}

impl<T> BBar for T where T: B {
    fn call_bar(&self) {
        self.bar()
    }
}

fn main() {
    let x = 42i32;
    x.call_foo();

    let y = 3.14f64;
    y.call_bar();
}