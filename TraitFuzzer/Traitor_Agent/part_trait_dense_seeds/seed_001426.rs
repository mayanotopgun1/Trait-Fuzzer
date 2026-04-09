#![allow(dropping_copy_types)]

trait DropExt<T> {
    fn drop_ext(self) -> impl core::fmt::Debug;
}

impl<T: std::fmt::Debug> DropExt<T> for T {
    fn drop_ext(self) -> impl core::fmt::Debug {
        self
    }
}

fn main() {
    use ::std::mem;
    let _ = 2_usize.drop_ext();
}