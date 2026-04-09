#![allow(unused_imports)]
#![deny(unused_qualifications)]

use self::A::B;

#[derive(PartialEq)]
pub enum A {
    B,
}

trait EnumTrait {
    fn is_b(&self) -> bool;
}

impl EnumTrait for A {
    fn is_b(&self) -> bool {
        matches!(self, A::B)
    }
}

fn main() {
    let a = A::B;
    println!("{}", a.is_b());
}