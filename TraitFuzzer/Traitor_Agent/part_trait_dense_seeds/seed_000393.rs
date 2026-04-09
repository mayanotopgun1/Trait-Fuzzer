#![feature(generic_associated_types, generic_const_exprs)]

struct GenericStruct<const T: usize> { val: i64 }

trait Convertible<const FROM: usize, const TO: usize> {
    type Out;
    fn convert(self) -> Self::Out;
}

impl<const T: usize> Convertible<T, {T + 1}> for GenericStruct<T> {
    type Out = GenericStruct<{T + 1}>;
    fn convert(self) -> Self::Out {
        GenericStruct { val: self.val }
    }
}

impl<const T: usize> Convertible<{T + 1}, T> for GenericStruct<{T + 1}> {
    type Out = GenericStruct<T>;
    fn convert(self) -> Self::Out {
        GenericStruct { val: self.val }
    }
}

pub fn main() {}