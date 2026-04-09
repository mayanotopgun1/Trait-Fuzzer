#![allow(dead_code)]
#![allow(non_camel_case_types)]

trait BarTrait {}
enum Bar {
    Nil,
}
impl BarTrait for Bar {}

fn foo() {
    fn zed(_z: &dyn BarTrait) {}
    fn baz() {
        zed(&Bar::Nil);
    }
}

pub fn main() {}