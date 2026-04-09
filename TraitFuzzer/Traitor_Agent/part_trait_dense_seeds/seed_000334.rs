#[global_allocator]
static A: std::alloc::System = std::alloc::System;

trait AllocLike {
    fn alloc(&self, layout: std::alloc::Layout) -> *mut u8;
}

impl AllocLike for std::alloc::System {
    fn alloc(&self, layout: std::alloc::Layout) -> *mut u8 {
        self.alloc(layout)
    }
}

fn main() {}