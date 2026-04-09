trait PtrDeref<T> { unsafe fn deref(&self) -> T; }

impl PtrDeref<u16> for *const u16 {
    unsafe fn deref(&self) -> u16 { **self }
}

fn main() {
    let ptr = 1 as *const u16;
    unsafe {
        let _ = ptr.deref();
    }
}