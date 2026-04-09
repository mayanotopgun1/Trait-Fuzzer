#![allow(noop_method_call)]
#![feature(specialization)]

mod x {
    pub use crate::y::*;
    pub use std::ops::Deref as _;
    use std::ops::Deref;

    pub trait DerefExt<T>: Deref<Target = T> {
        fn deref_ext(&self) -> &T;
    }

    impl<T, U> DerefExt<U> for T where T: Deref<Target = U> {
        default fn deref_ext(&self) -> &U {
            self.deref()
        }
    }
}

mod y {
    pub use crate::x::*;
    pub use std::ops::Deref as _;
}

pub fn main() {
    use x::*;
    (&0).deref_ext();
}