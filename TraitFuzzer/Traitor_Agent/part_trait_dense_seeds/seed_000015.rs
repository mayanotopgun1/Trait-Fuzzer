struct S<T>(*const T) where T: ?Sized;

trait AsPointer {
    type Target;
    fn as_ptr(&self) -> *const Self::Target;
}

impl<T> AsPointer for S<T> {
    type Target = T;
    fn as_ptr(&self) -> *const Self::Target {
        self.0
    }
}

fn main() {
    let u = vec![1, 2, 3];
    let _s: S<[u8]> = S(&u[..]);
}