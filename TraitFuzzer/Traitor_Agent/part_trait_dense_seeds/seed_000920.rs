#![feature(generic_associated_types)]

#[global_allocator]
static A: std::alloc::System = std::alloc::System;

use std::alloc::{GlobalAlloc, Layout};

trait AllocatorExt {
    unsafe fn allocate(&self, layout: Layout) -> *mut u8;
}

impl AllocatorExt for std::alloc::System {
    unsafe fn allocate(&self, layout: Layout) -> *mut u8 {
        self.alloc(layout)
    }
}

trait AllocatorView: AllocatorExt {
    unsafe fn allocate_twice(&self, layout: Layout) -> (*mut u8, *mut u8) {
        (self.allocate(layout), self.allocate(layout))
    }
}

impl<T: AllocatorExt> AllocatorView for T {}

fn main() {}