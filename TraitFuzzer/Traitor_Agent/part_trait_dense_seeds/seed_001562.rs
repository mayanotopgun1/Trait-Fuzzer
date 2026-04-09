trait CallOnceFn<T> { fn call(self) -> T; }
impl<F, T> CallOnceFn<T> for F where F: FnOnce() -> T {
    fn call(self) -> T {
        self()
    }
}

fn call_another_fn<T, F: CallOnceFn<T>>(f: F) -> T {
    f.call()
}

fn wub() -> ! {
    panic!("aah!");
}

fn main() {
    let x: i32 = call_another_fn(wub);
    let y: u32 = wub();
}