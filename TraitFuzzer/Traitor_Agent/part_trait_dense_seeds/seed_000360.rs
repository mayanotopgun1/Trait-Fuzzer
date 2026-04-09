#![deny(non_camel_case_types)]

fn main() {}

trait FooBar {
    #![allow(non_camel_case_types)]
}

impl FooBar for () {}

fn do_something(x: &dyn FooBar) {
    // Placeholder to avoid unused variable warning
    let _ = x;
}