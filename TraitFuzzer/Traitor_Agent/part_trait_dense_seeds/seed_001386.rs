#![crate_type = "rlib"]
#![crate_type = "dylib"]

pub trait StaticValue {
    fn value() -> u32;
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

    impl StaticValue for () {
        fn value() -> u32 {
            43
        }
    }
}