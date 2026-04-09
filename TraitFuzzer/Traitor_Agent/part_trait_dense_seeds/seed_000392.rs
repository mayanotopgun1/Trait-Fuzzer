#![feature(generic_const_exprs)]

struct GenericStruct<const T: usize> { val: i64 }

trait Convertible<const FROM: usize, const TO: usize> {
    fn convert(self) -> GenericStruct<TO>;
}

impl<const T: usize> Convertible<T, {T + 1}> for GenericStruct<T> {
    fn convert(self) -> GenericStruct<{T + 1}> {
        GenericStruct { val: self.val }
    }
}

impl<const T: usize> Convertible<{T + 1}, T> for GenericStruct<{T + 1}> {
    fn convert(self) -> GenericStruct<T> {
        GenericStruct { val: self.val }
    }
}

pub fn main() {}