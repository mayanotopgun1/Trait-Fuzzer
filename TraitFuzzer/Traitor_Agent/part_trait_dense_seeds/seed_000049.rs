#![allow(dead_code)]

use std::mem::{size_of, align_of};

trait SizeOf { fn size_of() -> usize; }
impl<T> SizeOf for T { fn size_of() -> usize { size_of::<T>() } }

trait AlignOf { fn align_of() -> usize; }
impl<T> AlignOf for T { fn align_of() -> usize { align_of::<T>() } }

#[repr(i8)]
enum Ei8 {
    Ai8 = 0,
    Bi8 = 1
}

#[repr(u8)]
enum Eu8 {
    Au8 = 0,
    Bu8 = 1
}

#[repr(i16)]
enum Ei16 {
    Ai16 = 0,
    Bi16 = 1
}

#[repr(u16)]
enum Eu16 {
    Au16 = 0,
    Bu16 = 1
}

#[repr(i32)]
enum Ei32 {
    Ai32 = 0,
    Bi32 = 1
}

#[repr(u32)]
enum Eu32 {
    Au32 = 0,
    Bu32 = 1
}

#[repr(i64)]
enum Ei64 {
    Ai64 = 0,
    Bi64 = 1
}

#[repr(u64)]
enum Eu64 {
    Au64 = 0,
    Bu64 = 1
}

#[repr(isize)]
enum Eint {
    Aint = 0,
    Bint = 1
}

#[repr(usize)]
enum Euint {
    Auint = 0,
    Buint = 1
}

#[repr(u8)]
enum Eu8NonCLike<T> {
    _None,
    _Some(T),
}

#[repr(i64)]
enum Ei64NonCLike<T> {
    _None,
    _Some(T),
}

#[repr(u64)]
enum Eu64NonCLike<T> {
    _None,
    _Some(T),
}

pub fn main() {
    assert_eq!(Ei8::size_of(), 1);
    assert_eq!(Eu8::size_of(), 1);
    assert_eq!(Ei16::size_of(), 2);
    assert_eq!(Eu16::size_of(), 2);
    assert_eq!(Ei32::size_of(), 4);
    assert_eq!(Eu32::size_of(), 4);
    assert_eq!(Ei64::size_of(), 8);
    assert_eq!(Eu64::size_of(), 8);
    assert_eq!(Eint::size_of(), isize::size_of());
    assert_eq!(Euint::size_of(), usize::size_of());
    assert_eq!(Eu8NonCLike::<()>::size_of(), 1);
    assert_eq!(Ei64NonCLike::<()>::size_of(), 8);
    assert_eq!(Eu64NonCLike::<()>::size_of(), 8);
    let u8_expected_size = round_up(9, Eu64NonCLike::<u8>::align_of());
    assert_eq!(Eu64NonCLike::<u8>::size_of(), u8_expected_size);
    let array_expected_size = round_up(28, Eu64NonCLike::<[u32; 5]>::align_of());
    assert_eq!(Eu64NonCLike::<[u32; 5]>::size_of(), array_expected_size);
    assert_eq!(Eu64NonCLike::<[u32; 6]>::size_of(), 32);

    assert_eq!(Eu32::align_of(), u32::align_of());
    assert_eq!(Eu64NonCLike::<u8>::align_of(), u64::align_of());
}

fn round_up(x: usize, a: usize) -> usize {
    ((x + (a - 1)) / a) * a
}