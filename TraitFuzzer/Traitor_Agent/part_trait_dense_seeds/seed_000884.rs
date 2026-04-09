#![feature(generic_associated_types)]

mod m1 {
    mod m2 {
        #[derive(Debug)]
        pub struct A;

        pub trait CreateA {
            type Out;
            fn create() -> Self::Out;
        }

        impl CreateA for A {
            type Out = A;
            fn create() -> Self::Out {
                A
            }
        }

        // Added a new trait to increase trait participation
        pub trait CreateATrait: CreateA {
            fn create_twice() -> <Self as CreateA>::Out {
                let a1 = Self::create();
                let a2 = Self::create();
                a2
            }
        }

        impl<T: CreateA> CreateATrait for T {}
    }
    use self::m2::{A, CreateA, CreateATrait};

    pub fn x() -> A {
        A::create_twice()
    }
}

fn main() {
    let x = m1::x();
    println!("{:?}", x);
}