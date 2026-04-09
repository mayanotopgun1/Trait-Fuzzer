#![feature(specialization)]

trait Sanitize {
    fn sanitize(&self);
}

trait BlockingSanitize: Sanitize {
    fn blocking(&self);
}

default impl<T> Sanitize for T {
    fn sanitize(&self) {}
}

default impl<T> BlockingSanitize for T {
    default fn blocking(&self) {
        println!("blocking call not detected");
    }
}

impl Sanitize for () {
    fn sanitize(&self) {}
}

impl BlockingSanitize for () {
    fn blocking(&self) {
        println!("specific blocking implementation");
    }
}

fn sanitizer_on() {
    let _: () = ();
    ().sanitize();
    ().blocking();
}

fn main() {
    sanitizer_on();
}