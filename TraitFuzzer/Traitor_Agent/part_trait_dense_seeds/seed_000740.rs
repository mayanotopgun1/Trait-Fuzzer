#![feature(core_intrinsics)]
#![feature(const_heap)]
#![feature(const_trait_impl)]

use std::intrinsics;

struct ZST;

const trait Allocate {
    unsafe fn const_allocate(size: usize, align: usize) -> *mut Self;
}

impl const Allocate for ZST {
    unsafe fn const_allocate(_size: usize, _align: usize) -> *mut ZST {
        intrinsics::const_allocate(0, 0) as *mut ZST
    }
}

fn main() {
    const {
        unsafe {
            let _ = <ZST as Allocate>::const_allocate(0, 0);
        }
    }
}