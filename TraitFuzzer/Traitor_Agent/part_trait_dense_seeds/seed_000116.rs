#![feature(specialization)]

trait Fooable { fn foo(self); }

impl<T> Fooable for T {
    default fn foo(self) {}
}

impl Fooable for *const () {
    fn foo(self) {}
}

fn foo(_: *const ()) {}

trait FooableExt: Fooable {
    fn bar(&self);
}

impl<T: Fooable + Copy> FooableExt for T {
    fn bar(&self) {
        (*self).foo();
    }
}

fn main() {
    let a = 3;
    (&a as *const _ as *const ()).bar();
}