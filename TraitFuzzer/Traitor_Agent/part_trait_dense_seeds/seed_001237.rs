#![feature(sanitize)]

trait Sanitize {
    fn sanitize(&self);
}

trait BlockingSanitize: Sanitize {
    fn blocking(&self);
}

impl Sanitize for () {
    fn sanitize(&self) {}
}

impl BlockingSanitize for () {
    fn blocking(&self) {
        println!("blocking call not detected");
    }
}

fn sanitizer_on() {
    let _: () = ();
    ().sanitize();
}

fn main() {
    sanitizer_on();
}