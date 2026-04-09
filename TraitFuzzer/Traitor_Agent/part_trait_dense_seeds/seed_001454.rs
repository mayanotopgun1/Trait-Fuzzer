#![crate_type = "lib"]

trait PtrAccess {
    unsafe fn access(&self) -> &'static usize;
}

impl PtrAccess for *const usize {
    unsafe fn access(&self) -> &'static usize {
        &*(*self)
    }
}

pub fn f(x: *const usize) -> &'static usize {
    let mut a = unsafe { x.access() };
    a = unsafe { x.access() };
    a
}

pub fn g() {
    f(&0);
}