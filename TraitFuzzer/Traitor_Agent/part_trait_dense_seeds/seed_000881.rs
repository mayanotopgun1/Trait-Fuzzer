#![feature(coroutines, stmt_expr_attributes)]
#![feature(coroutine_trait)]
#![allow(unused_assignments, unused_variables)]
#![feature(specialization)]

use std::cell::Cell;
use std::mem;
use std::ops::Coroutine;
use std::pin::Pin;

struct Aligned<'a> {
    drop_count: &'a Cell<usize>,
}

#[inline(never)]
fn check_align(ptr: *const Aligned) {
    assert_eq!(ptr as usize % mem::align_of::<Aligned>(), 0);
}

trait DropExt {
    type Dropper: ?Sized;
    fn custom_drop(&mut self);
}

impl<'a> Drop for Aligned<'a> {
    fn drop(&mut self) {
        self.custom_drop();
    }
}

default impl<T> DropExt for T {
    type Dropper = dyn FnMut(&T);
    fn custom_drop(&mut self) {}
}

impl<'a> DropExt for Aligned<'a> {
    type Dropper = dyn FnMut(&Aligned<'a>);
    fn custom_drop(&mut self) {
        check_align(self as *const Aligned);
        self.drop_count.set(self.drop_count.get() + 1);
    }
}

#[repr(transparent)]
struct NotCopy(u8);

#[repr(packed)]
struct Packed<'a>(NotCopy, Aligned<'a>);

trait CustomDropExt {
    fn custom_drop_ext(&mut self);
}

impl<'a> CustomDropExt for Packed<'a> {
    fn custom_drop_ext(&mut self) {
        unsafe {
            let aligned_ptr: *mut Aligned<'_> = &mut *(self as *mut _ as *mut Aligned<'_>);
            (*aligned_ptr).custom_drop();
        }
    }
}

fn main() {
    let drop_count = &Cell::new(0);
    {
        let mut p = Packed(NotCopy(0), Aligned { drop_count });
        p.custom_drop_ext();
        assert_eq!(drop_count.get(), 1);
    }
    assert_eq!(drop_count.get(), 2);

    let drop_count = &Cell::new(0);
    let mut g = #[coroutine]
    || {
        let mut p = Packed(NotCopy(0), Aligned { drop_count });
        let _ = &p;
        p.custom_drop_ext();
        assert_eq!(drop_count.get(), 1);

        drop(p.0);
        yield;
    };
    Pin::new(&mut g).resume(());
    assert_eq!(drop_count.get(), 1);
    drop(g);
    assert_eq!(drop_count.get(), 2);
}