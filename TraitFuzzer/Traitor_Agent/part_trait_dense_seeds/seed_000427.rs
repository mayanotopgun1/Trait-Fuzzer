use std::ptr;

trait PtrWrite {
    unsafe fn write_bytes(self, val: u8, count: usize);
}

impl<T> PtrWrite for *mut T {
    unsafe fn write_bytes(self, val: u8, count: usize) {
        ptr::write_bytes(self, val, count)
    }
}

fn main() {
    let mut dst = [0u16; 2];
    let mut dst_ptr = dst.as_mut_ptr();
    unsafe {
        #[cfg(null)]
        (*ptr::null_mut::<*mut u8>()).write_bytes(1u8, 2);
        #[cfg(misaligned)]
        dst_ptr.byte_add(1).write_bytes(1u8, 2);
    }
}