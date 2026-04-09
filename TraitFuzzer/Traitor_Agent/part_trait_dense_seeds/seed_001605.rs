#![feature(return_position_impl_trait_in_trait)]

trait PtrMut {
    type Item;
    unsafe fn deref(&self) -> &Self::Item;
    unsafe fn deref_mut(&mut self) -> &mut Self::Item;
}

impl PtrMut for *mut u32 {
    type Item = u32;

    unsafe fn deref(&self) -> &Self::Item {
        &*(*self)
    }

    unsafe fn deref_mut(&mut self) -> &mut Self::Item {
        &mut *(*self)
    }
}

fn main() {
    let mut ptr: *mut u32 = std::ptr::null_mut();
    unsafe {
        *(ptr.deref_mut()) = 42;
    }
}