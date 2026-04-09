#![allow(dead_code)]

trait Container<T> {}
impl<T> Container<T> for E<T> {}

enum E<T> { V(T) }

struct S<T>(T);

fn g<T: Container<U>, U>() -> bool {
    true
}

fn main() {
    let b = g::<E<isize>, isize>();
    assert!(b);
}