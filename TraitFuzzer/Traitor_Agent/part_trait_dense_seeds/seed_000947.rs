#![feature(specialization)]

trait Assoc {
    type Output;
}

default impl<T> Assoc for T {
    type Output = bool;
}

impl Assoc for u8 {}

trait Foo {}
impl Foo for u32 {}
impl Foo for <u8 as Assoc>::Output {}


trait IsSpecialized {
    fn is_specialized() -> bool;
}


default impl<T> IsSpecialized for T {
    fn is_specialized() -> bool { false }
}


impl IsSpecialized for u8 {
    fn is_specialized() -> bool { true }
}

impl IsSpecialized for u32 {
    fn is_specialized() -> bool { false }
}

fn main() {
    println!("u32 is specialized: {}", <u32 as IsSpecialized>::is_specialized());
    println!("u8 is specialized: {}", <u8 as IsSpecialized>::is_specialized());
}