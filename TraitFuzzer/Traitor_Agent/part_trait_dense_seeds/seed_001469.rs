#![allow(dead_code)]
#![allow(non_camel_case_types)]

use std::mem;

#[repr(packed)]
struct P1S4(u8,[u8;  3]);

#[repr(packed(2))]
struct P2S4(u8,[u8;  3]);

#[repr(packed)]
struct P1S5(u8, u32);

#[repr(packed(2))]
struct P2S6(u8, u32);

#[repr(packed)]
struct P1S13(i64, f32, u8);

#[repr(packed(2))]
struct P2S14(i64, f32, u8);

#[repr(packed(4))]
struct P4S16(u8, f32, i64, u16);

#[repr(C, packed(4))]
struct P4CS20(u8, f32, i64, u16);

enum Foo {
    Bar = 1,
    Baz = 2
}

#[repr(packed)]
struct P1S3_Foo(u8, u16, Foo);

#[repr(packed(2))]
struct P2_Foo(Foo);

#[repr(packed(2))]
struct P2S3_Foo(u8, u16, Foo);

#[repr(packed)]
struct P1S7_Option(f32, u8, u16, Option<Box<f64>>);

#[repr(packed(2))]
struct P2_Option(Option<Box<f64>>);

#[repr(packed(2))]
struct P2S7_Option(f32, u8, u16, Option<Box<f64>>);

trait AlignCheck {
    fn align(&self) -> usize;
    fn size(&self) -> usize;
}

impl AlignCheck for P1S4 {
    fn align(&self) -> usize { 1 }
    fn size(&self) -> usize { 4 }
}

impl AlignCheck for P1S5 {
    fn align(&self) -> usize { 1 }
    fn size(&self) -> usize { 5 }
}

impl AlignCheck for P1S13 {
    fn align(&self) -> usize { 1 }
    fn size(&self) -> usize { 13 }
}

impl AlignCheck for P1S3_Foo {
    fn align(&self) -> usize { 1 }
    fn size(&self) -> usize { 3 + mem::size_of::<Foo>() }
}

impl AlignCheck for P1S7_Option {
    fn align(&self) -> usize { 1 }
    fn size(&self) -> usize { 7 + mem::size_of::<Option<Box<f64>>>() }
}

impl AlignCheck for P2S4 {
    fn align(&self) -> usize { 1 }
    fn size(&self) -> usize { 4 }
}

impl AlignCheck for P2S6 {
    fn align(&self) -> usize { 2 }
    fn size(&self) -> usize { 6 }
}

impl AlignCheck for P2S14 {
    fn align(&self) -> usize { 2 }
    fn size(&self) -> usize { 14 }
}

impl AlignCheck for P4S16 {
    fn align(&self) -> usize { 4 }
    fn size(&self) -> usize { 16 }
}

impl AlignCheck for P4CS20 {
    fn align(&self) -> usize { 4 }
    fn size(&self) -> usize { 20 }
}

impl AlignCheck for P2S3_Foo {
    fn align(&self) -> usize { 2 }
    fn size(&self) -> usize { align_to(3 + mem::size_of::<P2_Foo>(), 2) }
}

impl AlignCheck for P2S7_Option {
    fn align(&self) -> usize { 2 }
    fn size(&self) -> usize { align_to(7 + mem::size_of::<P2_Option>(), 2) }
}

fn align_to(value: usize, align: usize) -> usize {
    (value + (align - 1)) & !(align - 1)
}

macro_rules! check {
    ($t:expr) => ({
        assert_eq!(mem::align_of_val(&$t), $t.align());
        assert_eq!(mem::size_of_val(&$t), $t.size());
    });
}

pub fn main() {
    let p1s4 = P1S4(0, [0; 3]);
    let p1s5 = P1S5(0, 0);
    let p1s13 = P1S13(0, 0.0, 0);
    let p1s3_foo = P1S3_Foo(0, 0, Foo::Bar);
    let p1s7_option = P1S7_Option(0.0, 0, 0, None);

    let p2s4 = P2S4(0, [0; 3]);
    let p2s6 = P2S6(0, 0);
    let p2s14 = P2S14(0, 0.0, 0);
    let p4s16 = P4S16(0, 0.0, 0, 0);
    let p4cs20 = P4CS20(0, 0.0, 0, 0);
    let p2s3_foo = P2S3_Foo(0, 0, Foo::Bar);
    let p2s7_option = P2S7_Option(0.0, 0, 0, None);

    check!(p1s4);
    check!(p1s5);
    check!(p1s13);
    check!(p1s3_foo);
    check!(p1s7_option);

    check!(p2s4);
    check!(p2s6);
    check!(p2s14);
    check!(p4s16);
    check!(p4cs20);
    check!(p2s3_foo);
    check!(p2s7_option);
}