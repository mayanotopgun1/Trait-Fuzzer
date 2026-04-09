use std::ptr;

trait PtrOps {
    fn addr_of(&self) -> *const Self;
    fn addr_of_mut(&mut self) -> *mut Self;
}

impl PtrOps for usize {
    fn addr_of(&self) -> *const Self {
        ptr::addr_of!(*self)
    }

    fn addr_of_mut(&mut self) -> *mut Self {
        ptr::addr_of_mut!(*self)
    }
}

trait RawPtrOps {
    fn raw_const(&self) -> *const Self;
    fn raw_mut(&mut self) -> *mut Self;
}

impl RawPtrOps for usize {
    fn raw_const(&self) -> *const Self {
        &raw const *self
    }

    fn raw_mut(&mut self) -> *mut Self {
        &raw mut *self
    }
}

static mut NOWHERE: usize = 0;

fn main() {
    let p2nowhere = unsafe { NOWHERE.addr_of() };
    let p2nowhere = unsafe { NOWHERE.addr_of_mut() };

    let raw2nowhere = unsafe { NOWHERE.raw_const() };
    let raw2nowhere = unsafe { NOWHERE.raw_mut() };
}