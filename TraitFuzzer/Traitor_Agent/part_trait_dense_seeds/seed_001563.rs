#![feature(impl_trait_in_assoc_type)]

trait CallOnceFn<T> { fn call(self) -> T; }
impl<F, T> CallOnceFn<T> for F where F: FnOnce() -> T {
    fn call(self) -> T {
        self()
    }
}

fn call_another_fn<T>(f: impl CallOnceFn<T>) -> T {
    f.call()
}

fn wub() -> ! {
    panic!("aah!");
}

fn main() {
    let _x: i32 = call_another_fn(wub);
}