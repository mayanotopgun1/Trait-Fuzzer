#![feature(impl_trait_in_assoc_type)]

trait Main {
    type Output: core::fmt::Debug + 'static;
    fn run(&self) -> Self::Output;
}

trait MainExt: Main {
    fn echo(&self) -> Box<dyn core::fmt::Debug> where Self::Output: Clone {
        Box::new(self.run().clone())
    }
}

impl<T> MainExt for T where T: Main {}

impl Main for () {
    type Output = ();
    fn run(&self) -> Self::Output {}
}

fn main() {
    let x: Box<dyn core::fmt::Debug> = <()>::echo(&());
    println!("{:?}", x);
}