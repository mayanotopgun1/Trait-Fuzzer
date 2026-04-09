#![feature(impl_trait_in_assoc_type)]
#![feature(generic_associated_types)]

trait Mainable {
    type Output<'a> where Self: 'a;
    fn run(&self) -> Self::Output<'_>;
}

trait Printable: Mainable {
    fn print(&self) where for<'a> Self::Output<'a>: std::fmt::Display {
        println!("{}", self.run());
    }
}

impl<T> Printable for T where T: Mainable {}

struct Program;

impl Mainable for Program {
    type Output<'a> = impl std::fmt::Display + 'a;
    fn run(&self) -> Self::Output<'_> { "Hello, World!" }
}

fn main() {
    let program = Program;
    program.print();
}