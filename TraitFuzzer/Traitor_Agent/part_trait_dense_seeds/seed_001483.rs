#![allow(unused_must_use)]
#![allow(unused_parens)]
#![feature(generic_associated_types)]

trait NumOps<T> {
    type Out;
    fn subtract(&self, other: T) -> Self::Out;
}

impl NumOps<i32> for i32 {
    type Out = i32;
    fn subtract(&self, other: i32) -> i32 {
        *self - other
    }
}

pub fn main() {
    let num = 12;

    assert_eq!(if (true) { 12 } else { 12 }.subtract(num), 0);
    assert_eq!(12.subtract(if (true) { 12 } else { 12 }), 0);
    if (true) { 12; }; {-num};
    if (true) { 12; }; {-num};
    if (true) { 12; };;; -num;
}