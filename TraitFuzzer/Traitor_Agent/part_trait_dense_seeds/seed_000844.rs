#![crate_type = "dylib"]
#![feature(trait_alias)]

trait DllTrait {}
impl DllTrait for () {}

fn do_something(x: Box<dyn DllTrait>) {
    // Placeholder for some functionality
}

fn main() {
    let x: Box<dyn DllTrait> = Box::new(());
    do_something(x);
}