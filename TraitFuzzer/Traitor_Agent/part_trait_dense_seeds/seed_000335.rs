trait Deref {
    type Target: ?Sized;
    fn deref(&self) -> &Self::Target;
}

impl<T> Deref for T
where
    T: core::ops::Deref,
{
    type Target = T::Target;
    fn deref(&self) -> &Self::Target {
        core::ops::Deref::deref(self)
    }
}

pub fn main() {
    let x = 3_usize;
    let ref y = x;
    assert_eq!(x, *y);
}