use std::any::{Any, TypeId};

trait DowncastExt: Any {
    fn safe_downcast<T: 'static>(&self) -> Option<&T>
    where
        Self: Sized,
    {
        if self.type_id() == TypeId::of::<T>() {
            unsafe { Some(&*(self as *const dyn Any as *const T)) }
        } else {
            None
        }
    }
}

impl<T: Any + Sized> DowncastExt for T {}

fn needs_usize(_: &usize) {}

fn main() {
    let x: &dyn Any = &1usize;
    if let Some(x) = x.safe_downcast::<usize>() {
        needs_usize(x);
    }
}