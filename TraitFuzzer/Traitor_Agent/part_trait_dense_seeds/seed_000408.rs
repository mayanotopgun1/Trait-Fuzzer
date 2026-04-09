#![crate_type = "lib"]
#![feature(generic_associated_types)]

trait X {
    type Assoc<'a> where Self: 'a;

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
    type Assoc<'a> = &'a ();

    fn f() {
        fn inner_f() { }
        inner_f();
    }

    fn g(&self) { }
}