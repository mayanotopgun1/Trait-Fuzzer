#![feature(type_alias_impl_trait)]

trait ByteAddPointer<T> {
    type Out;
    fn byte_add(self, offset: usize) -> Self::Out;
}

impl<T> ByteAddPointer<T> for *const T {
    type Out = Self;
    fn byte_add(self, offset: usize) -> Self::Out {
        unsafe { self.offset(offset as isize) }
    }
}

trait PointerOps<T>: ByteAddPointer<T> {}
impl<T, U> PointerOps<U> for T where T: ByteAddPointer<U> {}

fn main() {
    let x = [0u32; 2];
    let ptr = x.as_ptr();
    unsafe {
        let _ptr = &(*(ptr.byte_add(1)));
    }
}