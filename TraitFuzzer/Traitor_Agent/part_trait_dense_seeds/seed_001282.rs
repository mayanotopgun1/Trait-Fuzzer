#![feature(impl_trait_in_assoc_type)]

trait Main {
    type Output: core::fmt::Debug;
    fn run(&self) -> Self::Output;
}

trait MainExt: Main {
    fn echo(&self) -> Self::Output where Self::Output: Clone {
        self.run().clone()
    }
}

impl<T> MainExt for T where T: Main {}

impl Main for () {
    type Output = ();
    fn run(&self) -> Self::Output {}
}

fn main() {
    let x = <()>::echo(&());
    println!("{:?}", x);
}