#![allow(dead_code)]
#![allow(non_camel_case_types)]

trait Hax {
    fn dummy(&self) { }
}

impl<T> Hax for T { }

fn perform_hax<T: 'static>(x: Box<T>) -> Box<dyn Hax + 'static> {
    Box::new(x) as Box<dyn Hax + 'static>
}

trait ExtendedHax: Hax {
    fn extended_dummy(&self) { }
}

impl<U> ExtendedHax for U where U: Hax {}

fn deadcode() {
    perform_hax(Box::new("deadcode".to_string()));
}

pub fn main() {
    let _ = perform_hax(Box::new(42));
}