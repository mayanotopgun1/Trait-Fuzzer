#![feature(generic_associated_types)]

trait Deref {
    type Target<'a>: ?Sized where Self: 'a;
    fn deref<'a>(&'a self) -> &'a Self::Target<'a>;
}

impl<T> Deref for T
where
    T: core::ops::Deref,
{
    type Target<'a> = <T as core::ops::Deref>::Target where Self: 'a;
    fn deref<'a>(&'a self) -> &'a Self::Target<'a> {
        core::ops::Deref::deref(self)
    }
}

pub fn main() {
    let x = 3_usize;
    let ref y = x;
    assert_eq!(x, *y);
}