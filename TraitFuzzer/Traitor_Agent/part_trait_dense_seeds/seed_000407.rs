#![crate_type = "lib"]

trait X {
    fn x() {
        Self::f();
    }
    fn dummy(&self) {
        self.g();
    }

    fn f();
    fn g(&self);
}

impl X for () {
    fn f() {
        fn inner_f() { }
        inner_f();
    }

    fn g(&self) { }
}