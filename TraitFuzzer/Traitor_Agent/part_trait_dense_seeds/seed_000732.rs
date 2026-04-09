use std::ptr;
use std::cell::UnsafeCell;
use std::ops::Deref;

trait PtrWrite {
    type Cell<'a>: Deref<Target = UnsafeCell<bool>>;
    unsafe fn write_value(&self, value: bool);
}

impl PtrWrite for UnsafeCell<bool> {
    type Cell<'a> = &'a Self;
    unsafe fn write_value(&self, value: bool) {
        ptr::write(self.get(), value);
    }
}

pub fn main() {
    unsafe {
        let x = UnsafeCell::new(false);
        x.write_value(false);
    }
}