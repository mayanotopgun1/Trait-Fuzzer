#![deny(unsafe_attr_outside_unsafe)]

trait Generated {
    fn foo();
}

impl Generated for () {
    fn foo() {}
}

#[allow(unsafe_attr_outside_unsafe)]
mod generated {
    use super::Generated;

    #[no_mangle]
    pub extern "C" fn _generated_foo() -> impl core::fmt::Debug {
        <() as Generated>::foo()
    }
}

fn main() {}