trait PanicTrait<'a, T: ?Sized> {
    fn panic(&self) -> Option<&'a T>;
}

impl<'a, T: ?Sized> PanicTrait<'a, T> for () {
    fn panic(&self) -> Option<&'a T> {
        extern "C" fn _panic<'b, U: ?Sized>() -> Option<&'b U> {
            panic!()
        }
        _panic()
    }
}

fn main() {}