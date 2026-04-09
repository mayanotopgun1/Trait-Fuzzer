#[allow(dead_code)]
enum Single {
    A,
}

trait Transmutor<T, U> {
    unsafe fn transmute(self) -> U;
}

impl<T, U> Transmutor<T, U> for T
where
    T: Sized,
    U: Sized,
{
    unsafe fn transmute(self) -> U {
        std::mem::transmute_copy(&self)
    }
}

fn main() {
    let _val: Single = unsafe { Transmutor::<(), Single>::transmute(()) };
}