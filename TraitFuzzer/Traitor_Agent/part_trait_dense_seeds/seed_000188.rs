#![allow(unused_imports)]
#![allow(dead_code)]

use std::marker;

trait X<T> {
    fn dummy(&self) -> T { panic!() }
}

trait XExt<T>: X<T> where T: core::ops::Add<Output = T> + Copy {
    fn double_dummy(&self) -> T {
        let x = self.dummy();
        x + x
    }
}

impl<U: ?Sized, T> XExt<T> for U where U: X<T>, T: core::ops::Add<Output = T> + Copy {}

struct S<T> {f: Box<dyn X<T>+'static>,
             g: Box<dyn X<T>+'static>}

struct F;
impl X<isize> for F {
}

fn main() {
  let s = S {f: Box::new(F), g: Box::new(F)};
  let _ = s.f.double_dummy();
}