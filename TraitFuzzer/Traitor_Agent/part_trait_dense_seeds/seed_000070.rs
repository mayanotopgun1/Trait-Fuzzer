#![feature(generic_associated_types)]

trait Printer<'a> {
    type Output;
    fn print(&self) -> Self::Output;
}

trait PrinterExt<'a>: Printer<'a> {
    fn custom_print(&self) -> Self::Output where Self: Sized {
        self.print()
    }
}

impl<'a, S> PrinterExt<'a> for S where S: Printer<'a> {}

struct HelloWorld;

impl<'a> Printer<'a> for HelloWorld {
    type Output = &'a str;
    fn print(&self) -> Self::Output {
        "Hello, world!"
    }
}

fn main() {
    let hello_world = HelloWorld;
    println!("{}", hello_world.custom_print());
}