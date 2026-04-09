#![feature(impl_trait_in_assoc_type)]

#[allow(dead_code)]
enum Single {
    A,
}

trait Transmutor<T, U> {
    type Output;
    unsafe fn transmute(self) -> Self::Output;
}

impl<T, U> Transmutor<T, U> for T
where
    T: Sized,
    U: Sized,
{
    type Output = U;

    unsafe fn transmute(self) -> Self::Output {
        std::mem::transmute_copy(&self)
    }
}

fn main() {
    let _val: Single = unsafe { Transmutor::<(), Single>::transmute(()) };
}