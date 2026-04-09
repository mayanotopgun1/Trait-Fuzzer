mod foo {
    pub mod bar {
        pub trait BazQuux {
            fn baz(&self);
            fn quux(&self);
        }

        impl BazQuux for () {
            fn baz(&self) {}
            fn quux(&self) {}
        }

        pub fn baz() { }
        pub fn quux() { }
    }
}

use foo::bar::{baz, quux};

pub fn main() { baz(); quux(); }