#![crate_type = "rlib"]
#![crate_type = "dylib"]
#![feature(specialization)]

pub trait StaticValue {
    fn value() -> u32;
}

default impl<T> StaticValue for T {
    default fn value() -> u32 {
        0
    }
}

impl StaticValue for () {
    fn value() -> u32 {
        43
    }
}

pub mod a {
    pub trait StaticValue {
        fn value() -> u32;
    }

    default impl<T> StaticValue for T {
        default fn value() -> u32 {
            0
        }
    }

    impl StaticValue for () {
        fn value() -> u32 {
            43
        }
    }
}