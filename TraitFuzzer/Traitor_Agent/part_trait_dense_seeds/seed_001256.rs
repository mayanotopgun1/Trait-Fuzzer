#![no_std]

extern crate std;

trait UnwrapOr<T> {
    fn unwrap(self) -> T;
}

impl<T> UnwrapOr<T> for Option<T> {
    fn unwrap(self) -> T {
        self.unwrap()
    }
}

fn main() {
    let a: Option<&str> = Some("foo");
    let _ = a.unwrap();
}