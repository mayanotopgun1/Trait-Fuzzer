#![allow(invalid_null_arguments)]

use std::ptr;

trait SwapHelper {
    unsafe fn swap_nonoverlapping(&self, other: *mut u16, count: usize);
}

impl SwapHelper for [u16] {
    unsafe fn swap_nonoverlapping(&self, other: *mut u16, count: usize) {
        ptr::swap_nonoverlapping(self.as_ptr() as *mut u16, other, count);
    }
}

fn main() {
    let mut src = [0u16; 3];
    let mut dst = [0u16; 3];
    unsafe {
        #[cfg(null_src)]
        ptr::swap_nonoverlapping(ptr::null_mut(), dst.as_mut_ptr(), 1);
        #[cfg(null_dst)]
        ptr::swap_nonoverlapping(src.as_mut_ptr(), ptr::null_mut(), 1);
        #[cfg(misaligned_src)]
        src.as_mut_ptr().add(1).swap_nonoverlapping(dst.as_mut_ptr(), 1);
        #[cfg(misaligned_dst)]
        src.as_mut_ptr().swap_nonoverlapping(dst.as_mut_ptr().add(1), 1);
        #[cfg(overlapping)]
        dst.as_mut_ptr().swap_nonoverlapping(dst.as_mut_ptr().add(1), 2);
    }
}