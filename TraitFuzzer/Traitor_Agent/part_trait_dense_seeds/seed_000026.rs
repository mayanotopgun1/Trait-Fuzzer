mod foo {
    pub trait Printer {
        fn print(&self, y: isize);
    }

    impl Printer for () {
        fn print(&self, y: isize) {
            println!("{}", y);
        }
    }

    pub fn x(y: isize) {
        ().print(y);
    }
}

mod bar {
    use crate::foo::{Printer, x};
    use crate::foo::x as z;

    trait Z {
        fn z_call(&self, y: isize);
    }

    impl Z for () {
        fn z_call(&self, y: isize) {
            z(y);
        }
    }

    pub fn thing() {
        x(10);
        ().z_call(10);
    }
}

pub fn main() { bar::thing(); }