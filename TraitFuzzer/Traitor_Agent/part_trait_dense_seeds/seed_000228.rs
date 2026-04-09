#![allow(noop_method_call)]

mod x {
    pub use crate::y::*;
    pub use std::ops::Deref as _;
    use std::ops::Deref; // Import Deref trait

    pub trait DerefExt<T>: Deref<Target = T> {
        fn deref_ext(&self) -> &T {
            self.deref()
        }
    }

    impl<T, U> DerefExt<U> for T where T: Deref<Target = U> {}
}

mod y {
    pub use crate::x::*;
    pub use std::ops::Deref as _;
}

pub fn main() {
    use x::*;
    (&0).deref_ext();
}