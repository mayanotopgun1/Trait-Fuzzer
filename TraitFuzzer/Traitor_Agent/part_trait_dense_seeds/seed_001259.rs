#![crate_name = "crateresolve1"]
#![crate_type = "lib"]
#![feature(specialization)]

trait Compute {
    fn compute(&self) -> isize;
}

default impl<T> Compute for T {
    fn compute(&self) -> isize {
        0
    }
}

impl Compute for () {
    fn compute(&self) -> isize {
        30
    }
}

pub fn f() -> isize {
    let _empty: () = ();
    _empty.compute()
}