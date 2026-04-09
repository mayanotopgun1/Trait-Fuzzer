#![allow(dead_code)]
#![feature(impl_trait_in_assoc_type)]

use std::mem;

trait Choice {
    fn choice_u64(c: bool, a: u64, b: u64) -> u64;
}

impl Choice for () {
    fn choice_u64(_: bool, a: u64, _: u64) -> u64 {
        a
    }
}

trait MaxMinSize {
    fn max_size() -> usize;
    fn min_size() -> usize;
}

impl MaxMinSize for u8 {
    fn max_size() -> usize {
        u8::MAX as usize
    }
    fn min_size() -> usize {
        u8::MIN as usize
    }
}

trait Align {
    fn align_of() -> usize;
}

impl Align for u8 {
    fn align_of() -> usize {
        1
    }
}

trait SizeOf {
    fn size_of() -> usize;
}

impl SizeOf for u8 {
    fn size_of() -> usize {
        std::mem::size_of::<u8>()
    }
}

trait UnionFields {
    type FieldA;
    type FieldB;
}

struct UnionStruct<A, B> {
    field_a: A,
    field_b: B,
}

impl<A, B> UnionStruct<A, B> {
    fn new_field_a(a: A) -> Self {
        UnionStruct { field_a: a, field_b: unsafe { std::mem::zeroed() } }
    }

    fn new_field_b(b: B) -> Self {
        UnionStruct { field_a: unsafe { std::mem::zeroed() }, field_b: b }
    }
}

fn main() {}