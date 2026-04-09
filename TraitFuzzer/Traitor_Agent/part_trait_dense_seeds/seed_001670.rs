#![allow(dead_code)]

trait BarExt {
    fn bar_ext(self);
}

impl<S> BarExt for S
where
    S: Sized,
{
    fn bar_ext(self) {}
}

fn foo<T>() where T: Default {
    let _box = Box::<T>::new(T::default());
    _box.bar_ext()
}

pub fn main() {}