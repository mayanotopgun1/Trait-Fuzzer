trait DerefLike { fn deref(&self) -> &Self::Target; type Target: ?Sized; }
impl<T> DerefLike for T where T: std::ops::Deref { fn deref(&self) -> &Self::Target { self.deref() } type Target = <T as std::ops::Deref>::Target; }

fn f<'a>(x: &'a isize) -> &'a isize {
    return &*x;
}

pub fn main() {
    let three = &3;
    println!("{}", *f(three));
}