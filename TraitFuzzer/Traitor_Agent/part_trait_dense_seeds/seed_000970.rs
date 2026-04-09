#![feature(return_position_impl_trait_in_trait)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]

struct thing { x: isize, }

trait ThingDrop {
    fn drop(&mut self);
}

impl Drop for thing {
    fn drop(&mut self) {}
}

impl ThingDrop for thing {
    fn drop(&mut self) {}
}

fn thing() -> impl ThingMethod {
    thing {
        x: 0
    }
}

trait ThingMethod {
    fn f(self);
}

impl ThingMethod for thing {
    fn f(self) {}
}

pub fn main() {
    let z = thing();
    z.f();
}