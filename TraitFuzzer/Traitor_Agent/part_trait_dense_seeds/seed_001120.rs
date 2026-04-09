#![allow(non_upper_case_globals)]
#![allow(overflowing_literals)]

trait FooTrait {
    fn foo(&self) -> isize;
}

impl FooTrait for () {
    fn foo(&self) -> isize {
        0xca7f000d
    }
}

struct Bar<F> where F: FnMut() -> Box<dyn FooTrait> { f: F }

static mut b : Bar<fn() -> Box<dyn FooTrait>> = Bar { f: || Box::new(()) };

pub fn main() {
    unsafe { assert_eq!((b.f)().foo(), 0xca7f000d); }
}