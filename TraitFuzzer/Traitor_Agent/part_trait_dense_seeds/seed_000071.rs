#![feature(specialization)]
#![allow(incomplete_features)]

trait Printer<'a> {
    type Output;
    fn print(&self) -> Self::Output;
}

trait PrinterExt<'a>: Printer<'a> {
    fn custom_print(&self) -> Self::Output where Self: Sized;
}

impl<'a, S> PrinterExt<'a> for S
where
    S: Printer<'a>,
{
    default fn custom_print(&self) -> Self::Output {
        self.print()
    }
}

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