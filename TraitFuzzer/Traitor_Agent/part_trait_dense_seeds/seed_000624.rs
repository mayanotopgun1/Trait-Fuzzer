#![feature(impl_trait_in_assoc_type)]

trait CallIt<T> { fn call_it(self: Box<Self>) -> T; }
impl<F, T> CallIt<T> for F where F: FnOnce() -> T {
    fn call_it(self: Box<Self>) -> T {
        self()
    }
}

fn main() {
    let s = "hello".to_owned();
    assert_eq!(&CallIt::call_it(Box::new(|| s)) as &str, "hello");
}