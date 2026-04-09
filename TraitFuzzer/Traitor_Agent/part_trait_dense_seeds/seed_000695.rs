#![allow(unused_assignments)]
#![allow(unused_variables)]

trait Fooable {
    fn foo(x: &isize);
}

impl Fooable for isize {
    fn foo(x: &isize) {
        let a = 1;
        match x {
            mut z => {
                z = &a;
            }
        }
    }
}

pub fn main() {
    <isize as Fooable>::foo(&1);
}