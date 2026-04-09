#![allow(dropping_copy_types)]

trait DropExt<T> {
    fn drop_ext(self);
}

impl<T> DropExt<T> for T {
    fn drop_ext(self) {}
}

fn main() {
    use ::std::mem;
    2_usize.drop_ext();
}