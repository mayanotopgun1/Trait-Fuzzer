#![feature(specialization)]

trait Defaultable {
    fn new_default() -> Self;
}

default impl<T> Defaultable for T {
    default fn new_default() -> Self {
        panic!("Default implementation should not be used.")
    }
}

impl<T: Default> Defaultable for T {
    fn new_default() -> Self {
        Self::default()
    }
}

fn _test() -> impl Defaultable {
    ()
}

fn main() {}