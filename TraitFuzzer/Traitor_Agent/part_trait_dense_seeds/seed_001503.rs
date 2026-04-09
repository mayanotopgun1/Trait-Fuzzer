#![allow(dead_code)]

use std::mem::{size_of, align_of};
use std::os::raw::c_int;

trait EnumSize { fn size(&self) -> usize; }

impl EnumSize for E1 {
    fn size(&self) -> usize { 8 }
}

impl EnumSize for E2 {
    fn size(&self) -> usize { 8 }
}

impl EnumSize for E3 {
    fn size(&self) -> usize { 6 }
}

impl EnumSize for E4 {
    fn size(&self) -> usize { 8 }
}

impl EnumSize for E5 {
    fn size(&self) -> usize { align_size(10, align_of::<u32>()) }
}

impl EnumSize for E6 {
    fn size(&self) -> usize { align_size(14, align_of::<u64>()) }
}

impl EnumSize for E7 {
    fn size(&self) -> usize { align_size(6 + c_enum_min_size(), align_of::<c_int>()) }
}

#[repr(C, u8)]
enum E1 {
    A(u8, u16, u8),
    B(u8, u16, u8)
}

#[repr(u8, C)]
enum E2 {
    A(u8, u16, u8),
    B(u8, u16, u8)
}

#[repr(u8)]
enum E3 {
    A(u8, u16, u8),
    B(u8, u16, u8)
}

#[repr(u16)]
enum E4 {
    A(u8, u16, u8),
    B(u8, u16, u8)
}

#[repr(u32)]
enum E5 {
    A(u8, u16, u8),
    B(u8, u16, u8)
}

#[repr(u64)]
enum E6 {
    A(u8, u16, u8),
    B(u8, u16, u8)
}

#[repr(C)]
enum E7 {
    A(u8, u16, u8),
    B(u8, u16, u8)
}

pub struct p0f_api_query {
    pub magic: u32,
    pub addr_type: u8,
    pub addr: [u8; 16],
}

pub fn main() {
    let e1 = E1::A(0, 0, 0);
    let e2 = E2::A(0, 0, 0);
    let e3 = E3::A(0, 0, 0);
    let e4 = E4::A(0, 0, 0);
    let e5 = E5::A(0, 0, 0);
    let e6 = E6::A(0, 0, 0);
    let e7 = E7::A(0, 0, 0);

    assert_eq!(e1.size(), 8);
    assert_eq!(e2.size(), 8);
    assert_eq!(e3.size(), 6);
    assert_eq!(e4.size(), 8);
    assert_eq!(e5.size(), align_size(10, align_of::<u32>()));
    assert_eq!(e6.size(), align_size(14, align_of::<u64>()));
    assert_eq!(e7.size(), align_size(6 + c_enum_min_size(), align_of::<c_int>()));

    assert_eq!(size_of::<p0f_api_query>(), 21);
}

fn align_size(size: usize, align: usize) -> usize {
    if size % align != 0 {
        size + (align - (size % align))
    } else {
        size
    }
}

fn c_enum_min_size() -> usize {
    #[repr(C)]
    enum E {
        A,
    }
    size_of::<E>()
}