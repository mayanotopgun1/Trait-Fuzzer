#![allow(unreachable_code)]

trait UnitTrait {}
impl UnitTrait for () {}

fn main() {
    return ();

    let x: () = ();
    x
}