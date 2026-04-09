#![feature(type_alias_impl_trait)]

type A = Box<dyn Call + 'static + Send + Sync>;

trait Call {
    fn call(&self, x: u8) -> u8;
}

impl<F> Call for F
where
    F: Fn(u8) -> u8,
{
    fn call(&self, x: u8) -> u8 {
        (self)(x)
    }
}

fn make_caller() -> A {
    Box::new(|x| x + 1)
}

fn main() {
    let caller = make_caller();
    println!("{}", caller.call(5));
}