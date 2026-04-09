#![allow(dead_code)]
trait Trait<T> {
    fn f(&self, x: T);
}

trait Greet: Trait<&'static str> {
    fn greet(&self, name: &'static str) {
        self.f(name);
    }
}

impl<T> Greet for T where T: Trait<&'static str> {}

#[derive(Copy, Clone)]
struct Struct {
    x: isize,
    y: isize,
}

trait Printer {
    fn print_greeting(&self, message: &str);
}

impl Printer for Struct {
    fn print_greeting(&self, message: &str) {
        println!("{}", message);
    }
}

impl Trait<&'static str> for Struct {
    fn f(&self, x: &'static str) {
        self.print_greeting(x);
    }
}

pub fn main() {
    let a = Struct { x: 1, y: 2 };
    let b: Box<dyn Greet> = Box::new(a);
    b.greet("Mary");
    let c: &dyn Greet = &a;
    c.greet("Joe");
}