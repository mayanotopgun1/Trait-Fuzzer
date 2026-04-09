#![deny(missing_copy_implementations)]

trait IteratorExt: Iterator {
    fn peek(&self) -> Option<Self::Item>
    where
        Self: Clone,
        Self::Item: Copy,
    {
        let mut iter = self.clone();
        iter.next()
    }

    fn map_to_string(self) -> impl Iterator<Item = String> + 'static
    where
        Self: Sized + 'static,
        Self::Item: ToString,
    {
        self.map(|item| item.to_string())
    }
}

impl<T> IteratorExt for T where T: Iterator + Clone {}

pub struct MyIterator {
    num: u8,
}

impl Iterator for MyIterator {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

pub struct Handle {
    inner: *mut (),
}

trait HasInner {
    unsafe fn get_inner(&self) -> *const ();

    fn as_string(&self) -> impl ToString + 'static
    where
        Self: Sized,
    {
        unsafe { format!("{:?}", self.get_inner()) }
    }
}

impl HasInner for Handle {
    unsafe fn get_inner(&self) -> *const () {
        self.inner
    }
}

pub struct Handle2 {
    inner: *const (),
}

impl HasInner for Handle2 {
    unsafe fn get_inner(&self) -> *const () {
        self.inner
    }
}

pub enum MaybeHandle {
    Ptr(*mut ()),
}

impl HasInner for MaybeHandle {
    unsafe fn get_inner(&self) -> *const () {
        match self {
            MaybeHandle::Ptr(ptr) => *ptr as *const (),
        }
    }
}

pub union UnionHandle {
    ptr: *mut (),
}

impl HasInner for UnionHandle {
    unsafe fn get_inner(&self) -> *const () {
        self.ptr
    }
}

pub struct Array([u8; 2048]);

fn main() {}