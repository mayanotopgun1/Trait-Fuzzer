#![feature(specialization)]

trait F {
    fn func(&self, _a: Vec<isize>);
}

default impl<T> F for T {
    default fn func(&self, _a: Vec<isize>) {}
}

impl F for () {
    fn func(&self, _a: Vec<isize>) {}
}

pub fn main() {
    let f_impl = ();
    f_impl.func(vec![1, 2, 3, 4, 5]);
}