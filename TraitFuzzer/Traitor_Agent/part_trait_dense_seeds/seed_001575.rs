mod a {
    pub trait ModuleA {
        fn f(&self);
        fn g(&self);
    }

    impl ModuleA for () {
        fn f(&self) {}
        fn g(&self) {}
    }
}

mod b {
    pub use crate::a::*;

    pub struct B;

    impl ModuleA for B {
        fn f(&self) {}
        fn g(&self) {}
    }
}

pub fn main() {
    let b = b::B;
    use a::ModuleA; // Import the trait to bring its methods into scope
    b.f();
    b.g();
}