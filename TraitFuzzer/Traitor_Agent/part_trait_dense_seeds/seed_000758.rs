#![feature(allocator_api)]
#![crate_type = "rlib"]
#![feature(type_alias_impl_trait)]

use std::alloc::{GlobalAlloc, System, Layout};
use std::sync::atomic::{AtomicUsize, Ordering};

trait AllocCounter {
    fn increment(&self);
}

impl AllocCounter for AtomicUsize {
    fn increment(&self) {
        self.fetch_add(1, Ordering::SeqCst);
    }
}

type AllocatorCounter = dyn AllocCounter;

pub struct A(pub Box<dyn AllocCounter>);

unsafe impl GlobalAlloc for A {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.0.increment();
        System.alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.0.increment();
        System.dealloc(ptr, layout)
    }
}