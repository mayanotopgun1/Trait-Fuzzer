#![feature(specialization)]

trait Panicable {
    fn trigger_panic(&self) -> !;
}

default impl<T> Panicable for T {
    fn trigger_panic(&self) -> ! {
        panic!("generic")
    }
}

impl Panicable for isize {
    fn trigger_panic(&self) -> ! {
        panic!("test")
    }
}

fn main() {
    let __isize: isize = 0;
    __isize.trigger_panic();
}