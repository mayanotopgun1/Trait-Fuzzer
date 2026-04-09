trait PtrOps {
    type Item;
    fn null() -> Self::Item;
}

impl PtrOps for *mut u32 {
    type Item = *mut u32;
    fn null() -> Self::Item {
        std::ptr::null_mut()
    }
}

fn main() {
    let ptr: *mut u32 = <*mut u32 as PtrOps>::null();
    let _ptr: &mut u32 = unsafe { &mut *ptr };
}