#![feature(specialization)]

#[derive(Clone, Default)]
struct MaybeCopy<T>(T);

impl Copy for MaybeCopy<u8> {}

trait CopyCheck { fn check_copy(&self); }

default impl<T> CopyCheck for T {
    fn check_copy(&self) {
        println!("Not specialized: {}", std::any::type_name::<Self>());
    }
}

impl<T: Copy> CopyCheck for T {
    default fn check_copy(&self) {
        println!("Default copy implementation: {}", std::any::type_name::<Self>());
    }
}

impl CopyCheck for MaybeCopy<u8> {
    fn check_copy(&self) {
        println!("Specialized copy implementation for MaybeCopy<u8>: {}", std::any::type_name::<Self>());
    }
}

fn main() {
    let x = MaybeCopy::default();
    x.check_copy();
    [MaybeCopy::default(); 13];
}