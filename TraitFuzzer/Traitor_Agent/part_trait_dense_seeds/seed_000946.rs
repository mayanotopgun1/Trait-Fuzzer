#![allow(dead_code)]
#![allow(non_camel_case_types)]

enum maybe<T> { nothing, just(T), }

trait MaybeTrait {
    fn inspect(&self);
}

trait MaybeInspectExt: MaybeTrait {
    fn detailed_inspect(&self) {
        self.inspect();
    }
}

impl<T> MaybeInspectExt for T where T: MaybeTrait {}

impl<T> MaybeTrait for maybe<T> {
    fn inspect(&self) {
        match self {
            maybe::nothing => println!("A"),
            maybe::just(_) => println!("B"),
        }
    }
}

fn foo(x: maybe<isize>) {
    x.detailed_inspect();
}

pub fn main() { }