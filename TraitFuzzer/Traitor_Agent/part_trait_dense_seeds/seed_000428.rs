#![feature(specialization)]

use std::ptr;

trait PtrWrite {
    unsafe fn write_bytes(self, val: u8, count: usize);
}

default impl<T> PtrWrite for *mut T {
    unsafe fn write_bytes(self, val: u8, count: usize) {
        ptr::write_bytes(self, val, count)
    }
}

impl<'a, T> PtrWrite for &'a mut *mut T {
    unsafe fn write_bytes(self, val: u8, count: usize) {
        ptr::write_bytes(*self as *mut T, val, count);
    }
}

fn main() {
    let mut dst = [0u16; 2];
    #[cfg(null)]
    unsafe {
        (*ptr::null_mut::<*mut u8>()).write_bytes(1u8, 2);
    }
    #[cfg(misaligned)]
    unsafe {
        let dst_ptr = dst.as_mut_ptr();
        dst_ptr.byte_add(1).write_bytes(1u8, 2);
    }
}