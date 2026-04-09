#![deny(non_camel_case_types)]

fn main() {}

trait FooBar {
    #![allow(non_camel_case_types)]
}

impl FooBar for () {}