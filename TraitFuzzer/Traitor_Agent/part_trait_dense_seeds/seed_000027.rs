mod foo {
    pub trait Printer {
        fn print(&self, y: isize);
    }

    impl Printer for () {
        fn print(&self, y: isize) {
            println!("{}", y);
        }
    }

    pub fn x(y: isize) -> impl core::fmt::Debug {
        ().print(y);
        ()
    }
}

mod bar {
    use crate::foo::{Printer, x};
    use crate::foo::x as z;

    trait Z {
        fn z_call(&self, y: isize) -> impl core::fmt::Debug;
    }

    impl Z for () {
        fn z_call(&self, y: isize) -> impl core::fmt::Debug {
            z(y)
        }
    }

    pub fn thing() -> impl core::fmt::Debug {
        x(10);
        ().z_call(10)
    }
}

pub fn main() { let _ = bar::thing(); }