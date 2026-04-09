#![feature(specialization)]

trait Foo: Send {
    fn fooify(&self) -> i32;
}

default impl<T> Foo for T
where
    T: Send,
{
    fn fooify(&self) -> i32 {
        0
    }
}

impl Foo for isize {
    fn fooify(&self) -> i32 {
        *self as i32
    }
}

pub fn main() {
    let x = 42_isize;
    println!("{}", x.fooify());
}