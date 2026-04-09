#![crate_name = "crateresolve1"]
#![crate_type = "lib"]

trait Compute {
    fn compute(&self) -> isize;
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