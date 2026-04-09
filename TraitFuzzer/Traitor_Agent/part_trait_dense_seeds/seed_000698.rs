#![allow(dead_code)]
#![allow(non_camel_case_types)]

trait Callable<F> where F: FnOnce() {
    fn call(self) -> ();
}

struct R<F>
where
    F: FnOnce(),
{
    field: F,
}

impl<F> Callable<F> for R<F>
where
    F: FnOnce(),
{
    fn call(self) -> () {
        (self.field)();
    }
}

pub fn main() {
    fn f() {}
    let _i = R { field: f as fn() };
    _i.call();
}